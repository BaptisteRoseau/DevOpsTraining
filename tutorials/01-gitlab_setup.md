# Setting Up A GitLab Server

__WARNING: WORK IN PROGRESS !__

Table of contents:

[TOC]

## Summary

As an IT company you may need to set up your own [GitLab](https://about.gitlab.com/)'s instance.

Doing so is very easy using a [GitLab container](https://hub.docker.com/r/gitlab/gitlab-ee/) following [GitLab's official documentation](https://docs.gitlab.com/ee/install/docker.html) with rootful [Podman](https://podman.io/) or [Docker](https://www.docker.com/), but when it comes to secure and give access to your GitLab instance there are lots of pitfalls to fall into.

This tutorial provides steps to set up your own secure GitLab server.

For this tutorial, we will assume that your company will run its GitLab instance at `1.2.3.4`, which domain name is `my.company.gitlab.server`, using the following port mapping:

- `2022` (host) -> `22` (container)
- `20443` (host) -> `443` (container)

You can of course change any of their values depending on your environment.

*Notes:*

- *Port 80 will not be forwarded because HTTPS only should be used.*
- *It is also strongly recommended using a VPN to make `1.2.3.4` only available to your coworkers, but this set up is outside the scope of this tutorial.*

## Create A gitlab User (Optional)

### Why Creating A New User

A great security practice is to restrict permissions and access to the strict necessary.

That is, you could create a `software` group on your server, and give its users no access whatsoever. Each user of `software` would only have access to its own home directory, and restrict its permissions from other users.

With that set up, each software on your server is isolated, and if an attacker manages to gain access through a software vulnerability, his access would be restricted to the software's home directory, and privilege escalation would be much harder.

### Set up The gitlab User

============
WIP

TODO: Create user and group to restrict permissions

- Group `software`
- Home directory `/home/gitlab`
- User `gitlab` with group `software` and home `/home/gitlab` and a password
- `chmod 700 -R $HOME`
- Remove sudo usage

DONE in ubuntu container

As root:

```cmd
groupadd software
mkdir /home/software
useradd -m gitlab
```

TODO: specify GID and remove recursively access to other users from the group

- `chmod 700 -R $HOME`

============

## Create An SSL/TLS Certificate

If you already have an SSL/TLS certificate, you can skip this step.

If not, we will create one using [Let's Encrypt](https://letsencrypt.org/fr/) to allow traffic over HTTPS for our GitLab server. You can run the following commands anywhere, even you local machine, as the `1.2.3.4` server only needs the resulting certificate and key files.

### Install Let's Encrypt

TODO

### Generate The Certificate

TODO

## Installing Podman

To install Podman, please follow [the documentation](https://podman.io/getting-started/installation).

On Debian 11+, this is as simple as using:

```bash
sudo apt-get -y install podman
```

### Add Docker Registry

To be able to pull images from [Docker Hub](https://hub.docker.com), you need to add its registry.

To do so, simply add these lines in `/etc/containers/registries.conf`:

```toml
unqualified-search-registries = ["docker.io"]

[[registry]]
location = "docker.io"
```

## Running Rootless GitLab

To run a GitLab container, we will follow [this official tutorial](https://docs.gitlab.com/ee/install/docker.html) with a few adjustments for rootless environment.

First, login as the user who will host the GitLab server. Here we will assume it is the `gitlab` user.

Then, choose a directory to store GitLab server's data. Here, we will assume that it is `$HOME/GitLabServer`.

```bash
export GITLAB_HOME=$HOME/GitLabServer
```

To make sure the image is running, its `config`, `logs` and `data` directories must exist. We will also secure them by removing access to other users.

```bash
mkdir -p $GITLAB_HOME/config $GITLAB_HOME/logs $GITLAB_HOME/data
chmod 700 -R $GITLAB_HOME
```

Then, run the container:

```bash
podman run --detach \
    --hostname my.company.gitlab.server \
    --publish 20443:443 \
    --publish 2080:80 \
    --publish 2022:22 \
    --publish 9418:9418 \
    --publish 9090:9090 \
    --publish 5000:5000 \
    --publish 20465:465 \
    --publish 9418:9418 \
    --name gitlab \
    --restart always \
    --volume $GITLAB_HOME/config:/etc/gitlab:Z \
    --volume $GITLAB_HOME/logs:/var/log/gitlab:Z \
    --volume $GITLAB_HOME/data:/var/opt/gitlab:Z \
    --shm-size 256m \
  gitlab/gitlab-ee:latest
```

More information about GitLab used ports [here](https://docs.gitlab.cn/14.0/omnibus/package-information/defaults.html#ports).

================================

TODO: SSL/TLS Set up: (Remove this part when done)

To configure SSL/TLS in a GitLab container, you can follow these steps:

1. Create SSL/TLS certificates and keys for your domain using a tool like Let's Encrypt or OpenSSL.

1. Copy the certificate and key files to a location on the host system, such as `/etc/gitlab/ssl`.

1. Update the `gitlab.rb` configuration file, which is located in the `/etc/gitlab` directory, to include the following lines:

```cmd
external_url 'https://your-domain.com'
nginx['ssl_certificate'] = "/etc/gitlab/ssl/your-domain.com.crt"
nginx['ssl_certificate_key'] = "/etc/gitlab/ssl/your-domain.com.key"
```

Replace `your-domain.com` with the actual domain name for your GitLab instance.

1. Reconfigure GitLab using the `gitlab-ctl reconfigure` command to apply the changes to the configuration file.

After completing these steps, your GitLab instance should be accessible over HTTPS using the SSL/TLS certificates you configured.

================================

You can see the server is running using `podman ps` or `podman logs -f gitlab` commands.

### Automatically Restart GitLab On Reboot

`--restart always` option allows your GitLab instance to restart on crash, but not on system reboot. To make it restart on reboot, use `systemd` with the following commands:

```bash
podman generate systemd --new --name gitlab -f
mkdir -p ~/.config/systemd/user/
mv container-gitlab.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable container-gitlab.service
```

These commands will:

1. Create a `systemd` service configuration file
1. Add this configuration in your user services
1. Enable the `container-gitlab` service

And now your GitLab instance will restart on reboot.

You can check the status of the service using:

```bash
$ systemctl --user status container-gitlab.service
○ container-gitlab.service - Podman container-gitlab.service
     Loaded: loaded (/home/gitlab/.config/systemd/user/container-gitlab.service; enabled; vendor preset: disabled)
     Active: inactive (dead)
       Docs: man:podman-generate-systemd(1)
```

Don't worry about the `Active: inactive (dead)`: the service will be effective on reboot because your container is already running.

## NGINX Reverse Proxy

### Why Using A Reverse Proxy

Because GitLab container is running rootless, ports are not actually bound on standard 80, 443 and 22, but the container is not aware of that. Because of this mapping, you will encounter very annoying issues such as:

- Cloning address that are not copy/pastable: you need to update the port
- [Git LFS](https://git-lfs.com/) unable to upload files
- Clicking on issues and merge request links result in 404 errors

To fix these issues, the port forwarding have to be transparent to the clients, hence the use of a reverse proxy to remap `my.company.gitlab.server`'s ports 80 and 443 to 2080 and 20443.

Note that port 22 remapping will be fixed using [SSH client configs](#ssh-client-configurations).

### Reverse Proxy Set up

NGINX container will run a root, so you can store its config anywhere unsensitive.

For this tutorial, we will store it in `/root/nginx/nginx.conf`:

```nginx
events {
  worker_connections  4096;
}

http {
 server {
   listen 1.2.3.4:443;
   listen [::1]:443 ipv6only=on;

   server_name my.company.gitlab.server;

   location / {
       proxy_pass http://1.2.3.4:20443;
       proxy_set_header Host $host;
   }
 }
}
```

Then, run the following command __as root__ to run the container:

```bash
podman run \
    --detach \
    --restart always \
    --network host \
    -v /root/nginx/nginx.conf:/etc/nginx/nginx.conf \
    --name nginx \
  nginx
```

### Automatically Restart NGINX On Reboot

[Same as GitLab](#automatically-restart-gitlab-on-reboot), we will use `systemd` but this time rootful.

Run the following command __as root__ on your server:

```cmd
podman generate systemd --new --name nginx -f
cp container-nginx.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable container-nginx
```

To verify the service has correctly been set up, use __as root__:

```cmd
$ systemctl status container-nginx
○ container-nginx.service - Podman container-nginx.service
     Loaded: loaded (/etc/systemd/system/container-nginx.service; enabled; vendor preset: enabled)
     Active: inactive (dead)
       Docs: man:podman-generate-systemd(1)
```

Same as for GitLab, don't worry about the `Active: inactive (dead)`.

## SSH Client Configurations

Finally, each of the clients (read coworker) who needs to interact with your GitLab instance have to add the following lines in their `~/.ssh/config` file:

```cmd
# GitLab Server
Host my.company.gitlab.server
  PreferredAuthentications publickey
  IdentityFile ~/.ssh/id_rsa.pub
  HostName 1.2.3.4
  User git
  Port 2022
```

For more information about what does this configuration, please read the [SSH config documentation](https://www.ssh.com/academy/ssh/config#commonly-used-configuration-options).

## Domain Name Client Configuration

If the domain name `my.company.gitlab.server` is not directly available to your users using a VPN or a DNS, each user must add it in their DNS.

On Linux, the simplest way is to add the following line in `/etc/hosts`:

```cmd
1.2.3.1 my.company.gitlab.server
```
