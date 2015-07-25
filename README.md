# Web Bluetooth Registries

This repository holds files of metadata about Bluetooth entities. The
[Web Bluetooth specification](https://github.com/WebBluetoothCG/registries)
refers to this repository for information that may change after it's published.

## Blacklist

[gatt_blacklist.txt](/WebBluetoothCG/registries/blob/master/gatt_blacklist.txt)
holds a list of 128-bit GATT UUIDs that
websites using the Web Bluetooth API are forbidden from accessing.
This includes all of Services, Characteristics, and Descriptors.

This file contains comments in lines starting with `#`.
Non-comment lines contain a [valid UUID](https://webbluetoothcg.github.io/web-bluetooth/#dfn-valid-uuid)
followed optionally by a space and one of the tokens "`exclude-reads`" or "`excludes-writes`".
