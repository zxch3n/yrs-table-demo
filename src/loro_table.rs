use anyhow::Result;
use loro::{ExportMode, LoroDoc, LoroError, LoroList, LoroMap, LoroResult, LoroValue};

/// LoroTable represents a table using Loro CRDTs.
#[derive(Debug)]
pub struct LoroTable {
    doc: LoroDoc,
    cols: LoroList,
    rows: LoroList,
    cells: LoroMap,
}

pub struct Column {
    pub id: i64,
    pub name: String,
    pub width: u32,
}

pub struct Row {
    pub id: i64,
    pub height: u32,
}

pub struct Rows {
    rows: LoroList,
}

impl LoroTable {
    pub fn new(doc: LoroDoc) -> Self {
        let cols = doc.get_list("cols");
        let rows = doc.get_list("rows");
        let cells = doc.get_map("cells");
        Self {
            doc,
            cols,
            rows,
            cells,
        }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.cols.len()
    }

    pub fn import(&self, reader: &mut csv::Reader<std::fs::File>) -> LoroResult<u32> {
        let headers = reader
            .headers()
            .map_err(|e| LoroError::DecodeError(e.to_string().into()))?;
        let headers: Vec<_> = headers.iter().map(|s| s.to_string()).collect();

        let mut column_ids = Vec::with_capacity(headers.len());
        let mut column_id_index = std::collections::HashSet::with_capacity(headers.len());

        for header in headers {
            let col_id = loop {
                let id = fastrand::u32(..);
                if column_id_index.insert(id) {
                    break id;
                }
            };
            column_ids.push(col_id);
            self.cols.insert(
                self.cols.len(),
                loro::loro_value!({
                    "id": col_id as i64,
                    "name": header,
                    "width": 130
                }),
            )?;
        }

        let rows: Vec<_> = reader
            .records()
            .map(|record| {
                record
                    .map_err(|e| LoroError::DecodeError(e.to_string().into()))
                    .map(|r| r.iter().map(Self::parse).collect::<Vec<_>>())
            })
            // .take(100)
            .collect::<Result<Vec<_>, _>>()?;

        let mut row_ids = Vec::with_capacity(rows.len());
        let mut row_id_index = std::collections::HashSet::with_capacity(rows.len());

        for _ in 0..rows.len() {
            let row_id = loop {
                let id = fastrand::u32(..);
                if row_id_index.insert(id) {
                    break id;
                }
            };
            row_ids.push(row_id);
            self.rows.insert(
                0,
                loro::loro_value!({
                    "id": row_id as i64,
                    "height": 30
                }),
            )?;
        }

        let mut cell_count = 0;
        for (row_idx, record) in rows.into_iter().enumerate() {
            for (col_idx, cell) in record.into_iter().enumerate() {
                let cell_id = format!("{:x}:{:x}", row_ids[row_idx], column_ids[col_idx]);
                self.cells.insert(&cell_id, cell)?;
                cell_count += 1;
            }
        }
        Ok(cell_count)
    }

    fn parse(input: &str) -> LoroValue {
        if let Ok(n) = input.parse::<i64>() {
            LoroValue::I64(n)
        } else if let Ok(n) = input.parse::<f64>() {
            LoroValue::Double(n)
        } else {
            LoroValue::String(input.to_string().into())
        }
    }

    pub fn columns(&self) -> Vec<Column> {
        self.cols
            .get_value()
            .into_list()
            .unwrap()
            .iter()
            .filter_map(|col| {
                if let LoroValue::Map(map) = col {
                    Some(Column {
                        id: map.get("id").and_then(|v| v.as_i64()).cloned().unwrap_or(0),
                        name: map
                            .get("name")
                            .and_then(|v| v.as_string().map(|s| s.to_string()))
                            .unwrap_or_default(),
                        width: map
                            .get("width")
                            .and_then(|v| v.as_i64())
                            .cloned()
                            .unwrap_or(0) as u32,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn rows(&self) -> Rows {
        Rows {
            rows: self.rows.clone(),
        }
    }

    pub fn cols(&self) -> Vec<Column> {
        self.cols
            .get_value()
            .into_list()
            .unwrap()
            .iter()
            .map(|col| Column::from_loro_value(col.clone()))
            .collect()
    }

    pub fn encode(&self) -> Vec<u8> {
        self.doc.export(ExportMode::snapshot()).unwrap()
    }
}

impl Column {
    pub fn from_loro_value(value: LoroValue) -> Self {
        if let LoroValue::Map(map) = value {
            Column {
                id: map.get("id").and_then(|v| v.as_i64()).cloned().unwrap_or(0),
                name: map
                    .get("name")
                    .and_then(|v| v.as_string().map(|s| s.to_string()))
                    .unwrap_or_default(),
                width: map
                    .get("width")
                    .and_then(|v| v.as_i64())
                    .cloned()
                    .unwrap_or(0) as u32,
            }
        } else {
            panic!("Invalid column value");
        }
    }
}

impl Rows {
    pub fn iter(&self) -> impl Iterator<Item = Row> + '_ {
        let mut i = 0;
        std::iter::from_fn(move || {
            let row = self.rows.get(i);
            i += 1;
            row.map(|row| Row::new_from_map(row.into_value().unwrap()))
        })
    }
}

impl Row {
    pub fn new_from_map(map: LoroValue) -> Self {
        let LoroValue::Map(map) = map else {
            unreachable!()
        };
        Row {
            id: map
                .get("id")
                .map(|v| v.clone().into_i64().unwrap())
                .unwrap_or(0),
            height: map
                .get("height")
                .map(|v| v.clone().into_i64().unwrap())
                .unwrap_or(0) as u32,
        }
    }

    pub fn read_cells(&self, table: &LoroTable) -> Vec<LoroValue> {
        let mut cells = Vec::with_capacity(table.col_count());
        for col in table.cols() {
            let cell_id = format!("{:x}:{:x}", self.id, col.id);
            let cell = table.cells.get(&cell_id).unwrap();
            cells.push(cell.into_value().unwrap());
        }
        cells
    }
}
