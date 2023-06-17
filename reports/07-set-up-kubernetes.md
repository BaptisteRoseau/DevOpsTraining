# Set Up Kubernetes

I recently bought _The Kubernetes Book_ written by Nigel Poulton to help me learn this insanely powerful tool that is Kubernetes.

This report will most likely be my progression over is tutorials.

## Installing Kubernetes

To be able to run a Kubernetes cluster, I installed VMWare to simulate nodes that will run on Ubuntu 22.04. First I need to install:

- [Docker](https://www.docker.com/)
- [k3s](https://github.com/k3s-io/k3s)
- [k3d](https://k3d.io/)

I will later switch to Podman instead of Kubernetes but not while I am learning the basics.

### Install Docker

Easy, follow the [documentation](https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository):

```bash
sudo apt-get update
sudo apt-get install -y ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg
echo \
  "deb [arch="$(dpkg --print-architecture)" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  "$(. /etc/os-release && echo "$VERSION_CODENAME")" stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo docker run hello-world
```

Got the following:

```bash
Hello from Docker!
This message shows that your installation appears to be working correctly.
```

Done.

### Install `k3s`

`k3s` is a lightweight version of Kubernetes but fully conforms with it.
To install it, follow the [installation script](https://github.com/k3s-io/k3s#quick-start---install-script):

```bash
curl -sfL https://get.k3s.io | sh -
```

First unexpected error:

```log
[INFO]  systemd: Starting k3s
Job for k3s.service failed because the control process exited with error code.
See "systemctl status k3s.service" and "journalctl -xeu k3s.service" for details.
```

The `systemctl status k3s.service` command returned:

```bash
baptiste@baptiste-virtual-machine:~$ systemctl status k3s.service
● k3s.service - Lightweight Kubernetes
     Loaded: loaded (/etc/systemd/system/k3s.service; enabled; vendor preset: enabled)
     Active: activating (auto-restart) (Result: exit-code) since Tue 2023-06-13 20:02:27 CEST; 1s ago
       Docs: https://k3s.io
    Process: 47581 ExecStartPre=/bin/sh -xc ! /usr/bin/systemctl is-enabled --quiet nm-cloud-setup.service (code=exited, status=0/SUCCESS)
    Process: 47583 ExecStartPre=/sbin/modprobe br_netfilter (code=exited, status=0/SUCCESS)
    Process: 47584 ExecStartPre=/sbin/modprobe overlay (code=exited, status=0/SUCCESS)
    Process: 47585 ExecStart=/usr/local/bin/k3s server (code=exited, status=1/FAILURE)
   Main PID: 47585 (code=exited, status=1/FAILURE)
        CPU: 2.081s

juin 13 20:02:27 baptiste-virtual-machine systemd[1]: k3s.service: Consumed 2.081s CPU time.
```

Restarting the service did not work to I instead try to figure out what was going on using:

```bash
baptiste@baptiste-virtual-machine:~$ sudo /usr/local/bin/k3s server
...
FATA[0000] starting kubernetes: preparing server: init cluster datastore and https: listen tcp :6443: bind: address already in use 
```

But who is using that port ?

```bash
baptiste@baptiste-virtual-machine:~$ sudo lsof -i -P -n | grep LISTEN | grep 6443
kubelite  28110            root    3u  IPv6 123698      0t0  TCP *:16443 (LISTEN)
k3s-serve 59692            root   17u  IPv6 229740      0t0  TCP *:6443 (LISTEN)
```

Seems like the installation was successful after all, maybe a reboot will suffice.

```bash
baptiste@baptiste-virtual-machine:~$ systemctl status k3s.service
● k3s.service - Lightweight Kubernetes
     Loaded: loaded (/etc/systemd/system/k3s.service; enabled; vendor preset: enabled)
     Active: active (running) since Tue 2023-06-13 20:10:32 CEST; 47s ago
       Docs: https://k3s.io
    Process: 1161 ExecStartPre=/bin/sh -xc ! /usr/bin/systemctl is-enabled --quiet nm-cloud-setup.service (code=exited, status=0/SUCCESS)
    Process: 1165 ExecStartPre=/sbin/modprobe br_netfilter (code=exited, status=0/SUCCESS)
    Process: 1192 ExecStartPre=/sbin/modprobe overlay (code=exited, status=0/SUCCESS)
   Main PID: 1194 (k3s-server)
      Tasks: 87
     Memory: 1.4G
        CPU: 15.520s
    (...)
```

Yup, a good old reboot after playing with `systemctl` is usually useful, but can I run `kubectl` ?

```bash
baptiste@baptiste-virtual-machine:~$ sudo kubectl version --short
Client Version: v1.26.5+k3s1
Kustomize Version: v4.5.7
Server Version: v1.26.5+k3s1
```

Done.

TLDR: Use `curl -sfL https://get.k3s.io | sh -` then reboot to install `k3s`.

### Install `k3d`

I also made some research about [k3d](https://k3d.io/v5.5.1/), which is basically a Docker-based `k3s` wrapper allowing to run multiple-node cluster on a single machine using containers. It can become handy if I need multiple nodes without needing to set many VMs up.

To install it, follow the [documentation](https://k3d.io/v5.5.1/#installation):

```bash
curl -s https://raw.githubusercontent.com/k3d-io/k3d/main/install.sh | bash
```

Done.

## Set Up A VM Worker Node

First, I created a new Ubuntu VM and installed Docker and k3s on it... and that is my first mistake !

`k3s` should be installed as an agent on the node, so the command on the worker node to install it is:

```bash
curl -sfL https://get.k3s.io | K3S_URL=https://my_server:6443 K3S_TOKEN=my_node_token sh -
```

Where `my_server` is the IP of the control pane and `my_node_token` is actually the token found in the control pane's `/var/lib/rancher/k3s/server/token` file. Let's see if my node has been found:

```bash
baptiste@control-pane:~$ sudo kubectl get nodes
NAME                       STATUS   ROLES                  AGE     VERSION
baptiste-virtual-machine   Ready    control-plane,master   41m     v1.26.5+k3s1
node1                      Ready    <none>                 8m16s   v1.26.5+k3s1
```

Perfect. I `k3s` compatible with `k3d` if I create a cluster using the latter ?

```bash
sudo k3d cluster create tkb
```

```bash
baptiste@control-pane:~$ sudo kubectl get nodes
NAME                       STATUS   ROLES                  AGE   VERSION
node1                      Ready    <none>                 30m   v1.26.5+k3s1
baptiste-virtual-machine   Ready    control-plane,master   64m   v1.26.5+k3s1
```

Sadly not, that was just a quick test we will come back on their compatibility later if necessary.
