# Set Up Kubernetes

I recently bought _The Kubernetes Book_ written by Nigel Poulton to help me learn this insanely powerful tool that is Kubernetes.

This report will most likely be my progression over is tutorials.

## Installing Kubernetes

To be able to run a Kubernetes cluster, I installed VMWare to simulate nodes that will run on Ubuntu 22.04. First I need to install:

- Docker
- kubectl
- k3d

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

