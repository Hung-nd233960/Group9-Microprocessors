
| Name | Description | Default value | Allowed value |
|------|-------------|---------------|---------------|
|**ESP_HAL_CONFIG_PLACE_SPI_DRIVER_IN_RAM**|Places the SPI driver in RAM for better performance|false|-
|**ESP_HAL_CONFIG_PLACE_SWITCH_TABLES_IN_RAM**|Places switch-tables, some lookup tables and constants related to interrupt handling into RAM - resulting in better performance but slightly more RAM consumption.|true|-
|**ESP_HAL_CONFIG_PLACE_ANON_IN_RAM**|Places anonymous symbols into RAM - resulting in better performance at the cost of significant more RAM consumption. Best to be combined with `place-switch-tables-in-ram`.|false|-
|**ESP_HAL_CONFIG_XTAL_FREQUENCY**|The frequency of the crystal oscillator, in MHz. Set to `auto` to automatically detect the frequency. `auto` may not be able to identify the clock frequency in some cases. Also, configuring a specific frequency may increase performance slightly.|40|Any of ["40"]
|**ESP_HAL_CONFIG_SPI_ADDRESS_WORKAROUND**|(ESP32 only) Enables a workaround for the issue where SPI in half-duplex mode incorrectly transmits the address on a single line if the data buffer is empty.|true|-
|**ESP_HAL_CONFIG_FLIP_LINK**|(ESP32-C6/ESP32-H2 only): Move the stack to start of RAM to get zero-cost stack overflow protection.|false|-
|**ESP_HAL_CONFIG_PSRAM_MODE**|(ESP32, ESP32-S2 and ESP32-S3 only, `octal` is only supported for ESP32-S3) SPIRAM chip mode|quad|Any of ["quad", "octal"]
