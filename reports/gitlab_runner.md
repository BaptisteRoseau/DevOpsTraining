# Set Up A GitLab Runner

I tried to install GitLab Runner in a rootless Podman container for security purposes.

GitLab's most powerful tool are CI/CD pipelines. Those pipelines are defined in a `.gitlab-ci.yml` file in the root of a project, but they need to run somewhere.

That "somewhere" is any environment provided a [_GitLab Runner_](https://docs.gitlab.com/runner/) is installed and configure with any of:

- A shell executor
- An OCI container (usually Docker)
- A VM manager ([VirtualBox](https://www.virtualbox.org/))

In a company, there can be a GitLab Runner cluster managed via Kubernetes in a cloud so that users can directly write pipelines and skip the GitLab Runner setup for example.

I will not use a shell executor as it is less secure and can have side effects on my environment.

However, I can install a rootful GitLab Runner on my host, in a container or even in a VM.

## Install GitLab Runner On Host

To install GitLab Runner on my host, I simply followed GitLab's [documentation](https://docs.gitlab.com/runner/install/).

In short, I simply ran the following commands for a Debian (`.deb`) installation:

```cmd
arch=$(dpkg --print-architecture)
curl -LJO "https://gitlab-runner-downloads.s3.amazonaws.com/latest/deb/gitlab-runner_${arch}.deb"
sudo dpkg -i gitlab-runner_${arch}.deb
sudo gitlab-runner start
```

This will install GitLab Runner as root and make it run as a service .

Additionally, you can install GitLab Runner as root but make it run scripts as a user using the [following commands](https://docs.gitlab.com/runner/install/linux-manually.html#install-1):

```cmd
sudo useradd --comment 'GitLab Runner' --create-home gitlab-runner --shell /bin/bash
sudo gitlab-runner install --user=gitlab-runner --working-directory=/home/gitlab-runner
sudo gitlab-runner start
```

### Register A Runner

```cmd
gitlab-runner register \
  --name your-runner-name \
  --url https://shynamo-gitlab.com/ \
  --docker-image ubuntu:latest \
  --non-interactive \
  --executor docker \
  --registration-token ${REGISTRATION_TOKEN}
```

### Rootless Podman Configuration

To be able to use my rootless Podman configuration, there are a few things to configure.

There is a great blog written by Jean-Christophe Vassort on this specific topic here: <https://jcvassort.open-web.fr/gitlab-runner-replace-docker-with-podman/>. However, he uses a container for GitLab Runner and a dedicated podman user which is not our use case here (yet).

#### Enable The Podman Daemon

First, you need to set up the rootless Podman daemon for your user.

```cmd
systemctl enable podman.service
systemctl start podman.service
```

This will create a socket in `/run/user/$(id -u)/podman/podman.sock` that will be used by the runner.

#### Update The Runner Configuration

Then, add in the `/etc/gitlab-runner/config.toml` file:

```toml
[[runners]]
  ...
  executor = "docker"
  environment = ["FF_NETWORK_PER_BUILD=1"]
  [runners.docker]
    ...
    host = "unix:///run/user/1000/podman/podman.sock"
```

This will use the Podman rootless daemon's socket of my user (UID 1000), and force the creation of a new network per job. The latter is currently required by Podman to be able to run jobs.

Here my UID is 1000, but you can retrieve yours using:

```cmd
$ id -u
1000
```

Update the runner using:

```cmd
sudo gitlab-runner restart
```

And you are ready to go ! This runner will use your user's container registry to run GitLab's jobs.

## Install GitLab Runner In A Rootless Container

TODO
