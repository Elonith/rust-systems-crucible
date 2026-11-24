# Test draft

## 38 happy-path cases

| family      | command                    | expected output | how I got the expected value   |
| ----------- | -------------------------- | --------------- | ------------------------------ |
| temperature | cargo run 72 F C           | 22.22C          | (72 - 32) * 5 / 9              |
| temperature | cargo run 2579.95 K C      | 2306.80C        | 2579.95 - 273.15               |
| temperature | cargo run -143.95 C F      | -227.11F        | (-143.95 * 9 / 5) + 32         |
| temperature | cargo run 0 C K            | 273.15K         | 0 + 273.15                     |
| temperature | cargo run 298 C C          | 298.00C         | 298                            |
| temperature | cargo run -273.15 C K      | 0.00K           | -273.15 + 273.15               |
| temperature | cargo run 265 K F          | 17.33F          | (265 * 1.8) - 459.67           |
| temperature | cargo run 624.85 F K       | 602.51K         | (624.85 - 32) * 5 / 9 + 273.15 |
| length      | cargo run 385 mi Km        | 619.60Km        | 385 * 1.609344                 |
| length      | cargo run 8266.286 yd m    | 7558.69m        | 8266.286 * 0.9144              |
| length      | cargo run -837.38 ft m     | -255.23m        | -837.38 * 0.3048               |
| length      | cargo run 836.937 in cm    | 2125.82cm       | 836.937 * 2.54                 |
| length      | cargo run 1495 m ft        | 4904.86ft       | 1495 / 0.3048                  |
| length      | cargo run 820.29 yd yd     | 820.29yd        | 820.29 * 1                     |
| length      | cargo run 0.82 Km mi       | 0.51mi          | 0.82 / 1.609344                |
| length      | cargo run -1057.92 Km mi   | -657.36mi       | -1057.92 / 1.609344            |
| length      | cargo run 1200 Km ft       | 3937007.87ft    | 1200 * 1000 / 0.3048           |
| length      | cargo run -1849.26 ft Km   | -0.56Km         | -1849.26 * 0.0003048           |
| mass        | cargo run 5357.87 Kg lb    | 11812.08lb      | 5357.87 / 0.45359237           |
| mass        | cargo run -83638.67 lb Kg  | -37937.86Kg     | -83638.67 * 0.45359237         |
| mass        | cargo run 235.84 oz g      | 6685.95g        | 235.84 * 28.349523125          |
| mass        | cargo run 1083 Kg g        | 1083000.00g     | 1083 * 1000                    |
| mass        | cargo run 0.125 g g        | 0.12g           | 0.125 * 1                      |
| mass        | cargo run 2.675 g g        | 2.67g           | 2.675 * 1                      |
| mass        | cargo run -925.38 g oz     | -32.64oz        | -925.38 / 28.349523125         |
| mass        | cargo run 0 oz g           | 0.00g           | 0 * 28.349523125               |
| Data size   | cargo run 7484 KiB B       | 7663616.00B     | 7484 * 1024                    |
| Data size   | cargo run 280.28 MB GB     | 0.28GB          | 280.28 / 1000                  |
| Data size   | cargo run 4.7 GiB GB       | 5.05GB          | 4.7 * 1.073741824              |
| Data size   | cargo run 2.9 GB KiB       | 2832031.25KiB   | (2.9 * 1000000000) / 1024      |
| Data size   | cargo run 15.82 GB MiB     | 15087.13MiB     | 15.82 * (1000000000 / 1048576) |
| Data size   | cargo run 32 GiB GB        | 34.36GB         | 32 * 1.073741824               |
| Data size   | cargo run 37222.5376 B KiB | 36.35KiB        | 37222.5376 / 1024              |
| Data size   | cargo run 252.52 KB MB     | 0.25MB          | 252.52 / 1000                  |
| Data size   | cargo run 242.85 MB KB     | 242850.00KB     | 242.85 * 1000                  |
| Data size   | cargo run 285 KB KB        | 285.00KB        | 285 * 1                        |
| Volume      | cargo run 286 gal l        | 1082.63l        | 286 * 3.785411784              |
| Volume      | cargo run -1838.72 l gal   | -485.74gal      | -1838.72 / 3.785411784         |

## 9 failure cases

- **Wrong Usage**: exit code: 1.
- **Unknown Units**: exit code: 2.
- **Not available**: exit code: 3.
  If multiple errors occur: _Wrong Usage_ error wins.

| family        | command                   | exit code | the cause                       |
| ------------- | ------------------------- | --------- | ------------------------------- |
| Wrong usage   | cargo run 5 km kg         | 1         | Units from different families   |
| Wrong usage   | cargo run five mi cm      | 1         | Invalid number                  |
| Wrong usage   | cargo run five mi parsecs | 1         | Invalid number and unknown unit |
| Wrong usage   | cargo run                 | 1         | No arguments                    |
| Wrong usage   | cargo run 72 F C K        | 1         | Too many arguments              |
| Wrong usage   | cargo run 7 inc           | 1         | Insufficient arguments          |
| Unknown unit  | cargo run 88 mi parsecs   | 2         | Unit not found in the database  |
| Unknown unit  | cargo run 928 degree F    | 2         | Unit not found in the database  |
| Not available | cargo run 5 yd ft         | 3         | Conversion not yet available    |
