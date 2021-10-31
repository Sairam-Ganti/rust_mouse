// #include <bzlib.h>
// #include <linux/input.h>
// #include <linux/serio.h>
// #include <linux/libps2.h>
// #include <linux/slab.h>
// #include <linux/module.h>
// #include <asm/hypervisor.h>
// #include <asm/vmware.h>
// #include "psmouse.h"
// #include "vmmouse.h"
include!(concat!(env!("OUT_DIR"), "/vmmouse.rs"));
include!(concat!(env!("OUT_DIR"), "/psmouse.rs"));