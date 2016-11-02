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

This file contains comments in lines starting with `#`.
Non-comment lines contain a [valid UUID](https://webbluetoothcg.github.io/web-bluetooth/#valid-uuid)
followed optionally by a space and one of the tokens "`exclude-reads`" or "`exclude-writes`".
