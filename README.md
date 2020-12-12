# openwrt-exporter

Prometheus exporter of OpenWrt metrics.

It uses [ubus](https://openwrt.org/docs/techref/ubus) to collect statistics about the node, through [ubus-rs](http://github.com/bltavares/ubus-rs).

Metrics are only collected when scrapping, so it's a snapshot.

## Metrics Available

| Metric             | Type  | Labels |
|--------------------|-------|--------|
| wifi_clients_total | gauge | freq   |

## Build

```sh
cross build --target mips-unknown-linux-musl
```

## Release

Current release size stripped: ~1.2M

```sh
cross build --target mips-unknown-linux-musl --release
docker run --rm -v ${PWD}/target/mips-unknown-linux-musl/release:/target bltavares/rust-cross-mips-openwrt mips-linux-muslsf-strip /target/openwrt-exporter
scp target/mips-unknown-linux-musl/release/openwrt-exporter root@router.ip:~/openwrt-exporter
```
