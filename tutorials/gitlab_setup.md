# Setting Up A GitLab Server

As an IT company you may need to setup your own [GitLab](https://about.gitlab.com/)'s instance.

Doing so is very easy using a [GitLab container](https://hub.docker.com/r/gitlab/gitlab-ee/) with rootful [Podman](https://podman.io/) or [Docker](https://www.docker.com/), but when it comes to secure and give access to your GitLab instance there are lots of pitfalls to fall into.

This tutorial provides steps to setup your own secure GitLab server.

For this tutorial, we will assume that your company will run its GitLab instance at `1.2.3.4`, which domain name is `my.company.gitlab.server`, using the following port mapping:

- `2080` (host) -> `80` (container)
- `2022` (host) -> `22` (container)
- `20443` (host) -> `443` (container)

You can of course change any of there values depending on you environment.

*It is also strongly recommended to use a VPN to make `1.2.3.4` only available to your coworkers, but this setup is outside of the scope of this tutorial.*

## Create A gitlab User (Optional)

### Why Creating A New User ?

A great security practice is to restrict permissions and access to the script necessary.

That is, you could create a `software` group on your server, and give its users no access whatsoever. Each user of the `software` would only have access to its own home directory, and restrict its permissions from other users.

With that setup, each software on your server is isolated, and if an attacker manages to gain access through a software vulnerability, his access would be restricted to the software's home directory, and privilege escalation would be much harder.

### Setup The gitlab User

TODO: Create user and group to restrict permissions
- Group `software`
- Home directory `/home/gitlab`
- User `gitlab` with group `software` and home `/home/gitlab` and a password
- `chmod 700 -R $HOME`
- Remove sudo usage

DONE in ubuntu container

As root:

    groupadd software
    mkdir /home/software
    useradd -m gitlab

TODO: specify GID and remove recursively access to other users from the group

- `chmod 700 -R $HOME`

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

Then, choose a directory to store GitLab server's data. Here, we will assume that is is `$HOME/GitLabServer`.

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
  --publish 20443:443 --publish 2080:80 --publish 2022:22 \
  --name gitlab \
  --restart always \
  --volume $GITLAB_HOME/config:/etc/gitlab:Z \
  --volume $GITLAB_HOME/logs:/var/log/gitlab:Z \
  --volume $GITLAB_HOME/data:/var/opt/gitlab:Z \
  --shm-size 256m \
  gitlab/gitlab-ee:latest
```

You can see the server is running using `podman ps` or `podman logs -f gitlab` commands.

### Automatically Restart On Reboot

`--restart always` option allows your GitLab instance to restart on crash, but not on system reboot. To make it restart on reboot, use `systemd` with the following commands:

```bash
podman generate systemd --new --name gitlab -f
mkdir ~/.config/systemd/user/
mv container-gitlab.service ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable container-gitlab.service
rm container-gitlab.service
```

These commands will:

1. Create a `systemd` service configuration file
2. Add this configuration in your user services
3. Enable the `container-gitlab` service

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

### Why Using A Reverse Proxy ?

Because GitLab container is running rootless, ports are not actually bound on standard 80, 443 and 22, but the container is not aware of that. Because of this mapping, you will encounter very annoying issues such as:

- Cloning address that are not copy/pastable: you need to update the port
- [Git LFS](https://git-lfs.com/) unable to upload files
- Clicking on issues and merge request links result in 404 errors

To fix these issues, the port forwarding have to be transparent to the clients, hence the use of a reverse proxy to remap `my.company.gitlab.server`'s ports 80 and 443 to 2080 and 20443.

Note that port 22 remapping will be fixed using [SSH client configs](#ssh-client-configurations).

### Reverse Proxy Setup

TODO: Run NGINX Proxy

TODO: Do not forward HTTP port and always use HTTPS

## SSH Client Configurations

Finally, each of the clients (read coworker) who needs to interract with your GitLab instance have to add the following lines in their `~/.ssh/config` file:

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

## HTTPS Setup

(TODO)

This section has not been written and tested yet, but in production environment you simply cannot use HTTP and should **always** use HTTPS. You could even remove the HTTP `80` port in the GitLab's `--publish 2080:80` option, as well as in the NGINX configuration file after HTTPS setup.
