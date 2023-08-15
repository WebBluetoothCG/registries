# Web Bluetooth Registries

This repository holds files of metadata about Bluetooth entities. The
[Web Bluetooth specification](https://github.com/WebBluetoothCG/registries)
refers to this repository for information that may change after it's published
and defines the file formats here.

## Blocklist

[gatt_blocklist.txt](/gatt_blocklist.txt)
holds a list of 128-bit GATT UUIDs that
websites using the Web Bluetooth API are forbidden from accessing.
This includes all of Services, Characteristics, and Descriptors.

[manufacturer_data_blocklist.txt](/manufacturer_data_blocklist.txt)
holds a list of manufacturer data that websites using the Web Bluetooth API are forbidden from accessing.
Each entry of the list consists of a company identifier and data prefix that should be blocked.

These files contain comments in lines starting with `#`.
Non-comment lines contain one of these:
  * A [valid UUID](https://webbluetoothcg.github.io/web-bluetooth/#valid-uuid)
followed optionally by a space and one of the tokens "`exclude-reads`" or "`exclude-writes`".
  *  The word `manufacturer` followed by a space and a group of hexadecimal digits (at most four
  hexadecimal digits) that represents a company identifier, which can be found on the
  [Bluetooth Assigned Numbers website](https://www.bluetooth.com/specifications/assigned-numbers).
  A space after that is an advertising data prefix in the format `advdata-<data>/<mask>`, where
  `data` and `mask` are strings of hexadecimal digits of the same length. `data` is a sequence of bytes.
  A data prefix matches an advertising data if each byte of the prefix is equal to the corresponding byte
  of the advertising data after both are ANDed with the corresponding byte of the mask. For example,
  `advdata-001100/00FF00` would match advertising data such as `11` `11` `11`, or `00` `11` `22`, etc.

## Assigned numbers

[gatt_assigned_services.txt](/gatt_assigned_services.txt),
[gatt_assigned_characteristics.txt](/gatt_assigned_characteristics.txt), and
[gatt_assigned_descriptors.txt](/gatt_assigned_descriptors.txt) hold
respectively a list of human-readable names for standardized GATT services,
characteristics, and descriptors, and their 128-bit GATT UUIDs to use with the
Web Bluetooth API.

These files contain comments in lines starting with `#`. Non-comment lines
contain a [valid
name](https://webbluetoothcg.github.io/web-bluetooth/#valid-name) followed by a
space and a [valid
UUID](https://webbluetoothcg.github.io/web-bluetooth/#valid-uuid).