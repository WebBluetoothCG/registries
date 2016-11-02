# Policy for adding UUIDs to the GATT Blocklist

## UUIDs are added through Github issues in this repository

There is currently no minimum or maximum time between
when the issue is filed and when the new UUID is added to the blocklist.
If a device is actively being exploited, we'll try to make a decision quickly,
and implementations should also be able to deploy these changes quickly.

## Blocklisting by an attribute's creator is preferred

Ideally, we blocklist an attribute because the person or group who first used that UUID asks us to.
This sort of blocklisting can be done quickly, because there's little risk of disagreement
or collateral damage.

There is some risk,
since a device maker can re-use an attribute in a way that's
more secure than the original,
and blocklisting the original will also block use of the copy.
We encourage developers to use a new UUID
when they're improving the security of an attribute.

Sometimes, a third party will notice a vulnerability in someone else's service.
We're willing to blocklist these services, too, especially if the risk is high,
but we need to spend more time trying to find dissenting voices.

When the original use of a UUID is secure, but a copy is less secure,
we generally wouldn't blocklist the UUID.

## Blocklisting UUIDs used on only vulnerable devices is preferred

Block listing a UUID breaks all of the devices that include attributes with that UUID,
whether or not they're vulnerable to the risk that caused us to blocklist the attribute.
If a UUID is used for both vulnerable and safe attributes on different devices,
we can't protect the first group of devices without unnecessarily breaking the second.

There's necessarily a judgement call here,
that's resolved by an informal cost/benefit analysis.

## Pure communication protocols generally won't be blocklisted

A GATT service that encodes a communication protocol with no indication of its meaning
is too broadly usable to say whether the service itself is secure or insecure,
so we generally won't be able to blocklist the whole service
even if some devices are critically vulnerable to websites using that service.
We recommend that devices use precise UUIDs instead of generic ones
so that they can take advantage of blocklisting.

## Examples

### FIDO

[Issue #12](https://github.com/WebBluetoothCG/registries/issues/12)
asked us to blocklist the FIDO U2F service.
The request came from one of the editors of the
[FIDO specification](https://fidoalliance.org/specs/fido-u2f-bt-protocol-id-20150514.pdf),
and the protocol fundamentally doesn't meet its goals if it's accessed from a web page.
This was an easy UUID to blocklist.

### Firmware update

We've blocklisted 3 firmware update services so far,
even though the process was contrary to our preferences above.

A device that needs its user to perform some action on the device in order to accept an update
would not be vulnerable to attack via these services,
and neither would a device that checked
that updates are signed by the device's manufacturer.
However, devices that don't incorporate either of these defenses can accept an update
that would make the device permanently act against its owner's wishes.
Because the initial uses were insecure,
we concluded that more total uses were likely to be insecure than secure.

We blocklisted these services before device manufacturers asked us to,
on the theory that waiting would increase the number of devices that depended on web access
and so increase the damage vs blocklisting early.

Since blocklisting the insecure update services,
we've seen Nordic introduce a
[secure replacement](https://infocenter.nordicsemi.com/topic/com.nordic.infocenter.sdk5.v12.0.0/ble_sdk_app_dfu_bootloader.html)
under a new UUID,
which devices can use without risk of being blocklisted.
Note that a device that uses the new UUID in an insecure way
also won't get the benefit of blocklisting if someone starts attacking that device.
