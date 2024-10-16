# Yrs Loro table demo

This is a simple demo of a collaborative table capabilities using
[yrs](https://docs.rs/yrs/latest/yrs/) and
[loro](https://docs.rs/yrs/latest/yrs/) library. As an example we're using a
Uber data set, which can be downloaded from
[Kaggle](https://www.kaggle.com/datasets/yasserh/uber-fares-dataset#).

This data set contains 200 000 rows and in this demo we can see how we're able
to import it into Yrs & Loro document structure

## Example

```
> cargo run --release

============================
Running Yrs...
============================
imported 1800000 cells in 1.170876458s
encoded 1800000 cells (200000 rows x 9 columns) in 304.002583ms: 58041967 bytes, 11737639 compressed (original file size: 23458139 bytes) decoded in 1.325586167s

	key	fare_amount	pickup_datetime	pickup_longitude	pickup_latitude	dropoff_longitude	dropoff_latitude	passenger_count	
11951496	2010-05-15 04:08:00.00000076	14.1	2010-05-15 04:08:00 UTC	-73.98439499999999	40.720077	-73.985508	40.768793	1	
20259894	2015-05-20 14:56:25.0000004	14.5	2015-05-20 14:56:25 UTC	-73.99712371826173	40.7254524230957	-73.98321533203125	40.69541549682617	1	
27804658	2009-06-29 00:42:00.00000078	30.9	2009-06-29 00:42:00 UTC	-73.986017	40.756487	-73.85895699999999	40.692588	2	
16382965	2014-03-14 01:09:00.0000008	7.5	2014-03-14 01:09:00 UTC	-73.98472199999999	40.736837	-74.006672	40.73962	1	
42598914	2012-10-28 10:49:00.00000053	3	2012-10-28 10:49:00 UTC	-73.987042	40.739367	-73.986525	40.740297	1	
3189201	2014-01-31 14:42:00.000000181	12	2014-01-31 14:42:00 UTC	-73.98307	40.76077	-73.972972	40.754177	1	
28359558	2012-09-29 19:51:27.0000006	9.5	2012-09-29 19:51:27 UTC	-73.987798	40.72121	-73.98096	40.744388	1	
20566507	2010-01-30 16:24:00.000000199	8.9	2010-01-30 16:24:00 UTC	-74.003548	40.714045	-73.99105300000001	40.6845	1	
13512837	2015-06-08 10:49:14.0000001	17.5	2015-06-08 10:49:14 UTC	-73.98145294189453	40.743919372558594	-74.01390838623048	40.7126350402832	1	
9577367	2015-05-24 22:05:56.0000002	12	2015-05-24 22:05:56 UTC	-73.98710632324219	40.74189376831055	-73.95223999023438	40.77295684814453	1	



============================
Running Loro...
============================
imported 1800000 cells in 1.494787167s
encoded 1800000 cells (200000 rows x 9 columns) in 2.403824083s: 57830176 bytes, 36673478 compressed (original file size: 23458139 bytes) decoded in 352.786292ms

[ShallowMode] encoded 1800000 cells (200000 rows x 9 columns) in 927.799292ms: 37813078 bytes, 25078559 compressed (original file size: 23458139 bytes) decoded in 554.372417ms

	key	fare_amount	pickup_datetime	pickup_longitude	pickup_latitude	dropoff_longitude	dropoff_latitude	passenger_count	
I64(24238194)	String("2015-05-07 19:52:06.0000003")	Double(7.5)	String("2015-05-07 19:52:06 UTC")	Double(-73.99981689453125)	Double(40.73835372924805)	Double(-73.99951171875)	Double(40.72321701049805)	I64(1)	
I64(27835199)	String("2009-07-17 20:04:56.0000002")	Double(7.7)	String("2009-07-17 20:04:56 UTC")	Double(-73.994355)	Double(40.728225)	ouble(-73.99471)	Double(40.750325)	I64(1)	
I64(44984355)	String("2009-08-24 21:45:00.00000061")	Double(12.9)	String("2009-08-24 21:45:00 UTC")	Double(-74.005043)	Double(40.74077)	ouble(-73.962565)	Double(40.772647)	I64(1)	
I64(25894730)	String("2009-06-26 08:22:21.0000001")	Double(5.3)	String("2009-06-26 08:22:21 UTC")	Double(-73.976124)	Double(40.790844)	ouble(-73.965316)	Double(40.803349)	I64(3)	
I64(17610152)	String("2014-08-28 17:47:00.000000188")	Double(16.0)	String("2014-08-28 17:47:00 UTC")	Double(-73.925023)	Double(40.744085)	ouble(-73.97308199999999)	Double(40.761247)	I64(5)	
I64(44470845)	String("2011-02-12 02:27:09.0000006")	Double(4.9)	String("2011-02-12 02:27:09 UTC")	Double(-73.96901899999999)	Double(40.75591)	Double(-73.96901899999999)	Double(40.75591)	I64(1)	
I64(48725865)	String("2014-10-12 07:04:00.0000002")	Double(24.5)	String("2014-10-12 07:04:00 UTC")	Double(-73.96144699999999)	Double(40.693965000000006)	Double(-73.871195)	Double(40.774297)	I64(5)	
I64(44195482)	String("2012-12-11 13:52:00.00000029")	Double(2.5)	String("2012-12-11 13:52:00 UTC")	Double(0.0)	Double(0.0)	Double(0.0)	ouble(0.0)	I64(1)	
I64(15822268)	String("2012-02-17 09:32:00.00000043")	Double(9.7)	String("2012-02-17 09:32:00 UTC")	Double(-73.975187)	Double(40.745767)	ouble(-74.00272)	Double(40.743536999999996)	I64(1)	
I64(50611056)	String("2012-03-29 19:06:00.000000273")	Double(12.5)	String("2012-03-29 19:06:00 UTC")	Double(-74.001065)	Double(40.741787)	ouble(-73.96304)	Double(40.775012)	I64(1)
```
