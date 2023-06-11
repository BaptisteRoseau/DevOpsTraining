# Install VMWare On Linux

I need to be able to run Virtual Machines on my PC to simulate a cluster of nodes and to make sure my tutorials are valid.

I tried [VirtualBox](https://www.virtualbox.org/) for 5 minutes before giving up to try to install a custom `.iso`.

Let's install [VMWare Workstation Player](https://customerconnect.vmware.com/fr/downloads/details?downloadGroup=WKST-PLAYER-1625&productId=1039&rPId=98565) instead.

_Keep in mind that for commercial use a **license** is required to use VMWare Workstation Player_

## Download And Install

Download then install the bundle [from this link](https://www.vmware.com/in/products/workstation-player.html), then use:

```bash
chmod +x ./VMware-Player-Full-*-*.bundle
sudo ./VMware-Player-Full-*-*.bundle
```

This will install VMware Player, and then I will be able to run multiple VMs to simulate a cluster !

![vmware_player_library](assets/vmware_player_library.png)

I first had issues with kernel headers not found by VMWare Player, but a little help from [this blog article](https://linux.how2shout.com/install-vmware-workstation-player-on-ubuntu-22-04-lts/) and updating the VMWare Player version solved this issue.

## Set Up An Ubuntu VM Template

First, I downloaded an Ubuntu 22.04 ISO [from the official website](https://ubuntu.com/download/desktop), created a VM with:

- 2 CPU cores
- 8 GiB RAM
- 40 Go Disk

That should be sufficient to run Kubernetes containers and even GitLab runners.

I also installed my required tool chain such as VSCodium and then made a copy of the VM to serve as a template.

That's it for my virtual machines tool chain setup !
