Mini Monkey Rust client
=======================

[![Build Status](https://travis-ci.org/mini-monkey/minimonkeyrs.svg?branch=master)](https://travis-ci.org/mini-monkey/minimonkeyrs)
[![License Apache 2](https://img.shields.io/badge/License-Apache2-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)

![Logo](doc/minimonkey_small.png)

![Mini Monkey](https://github.com/mini-monkey/mini-monkey) is a minimal message routing system.
Considerably smaller and simpler than [MQTT](https://en.wikipedia.org/wiki/MQTT).

This repository contains the rust crate for the Mini Monkey client code.

Project Plan
------------

- [ ] Stabile publish
- [ ] Stabile subscribe
- [ ] Minimal administration tasks
- [ ] Complete broker provisioning

Provisioning
------------

One of the main goals of the Mini Monkey broker is that it can be programmatically provisioned.

```sh
mm_provision --host localhost --port 1773 --token adminToken --file provision.yaml
```

```yaml
rooms:
  - name: Kitchen
    tokens_allowed_to_admin:
      - admin1
    tokens_allowed_to_publish:
      - sensor1
      - sensor2
    tokens_allowed_to_subscribe:
      - house_automation

  - name: LivingRoom
    tokens_allowed_to_admin:
      - admin2
    tokens_allowed_to_publish:
      - sensor3
      - sensor4
    tokens_allowed_to_subscribe:
      - house_automation
```
