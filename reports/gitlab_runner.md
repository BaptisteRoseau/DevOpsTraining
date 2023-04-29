# Setup a GitLab Runner

As I like to make things harder but more secure, I tried to install GitLab Runner in a rootless podman container.

## GitLab Runner On Host

To install GitLab Runner on my host, I simply followed GitLab's [documentation](https://docs.gitlab.com/runner/install/).

In short, I simply ran the following commands for a Debian (`.deb`) installation:

```cmd
arch=$(dpkg --print-architecture)
curl -LJO "https://gitlab-runner-downloads.s3.amazonaws.com/latest/deb/gitlab-runner_${arch}.deb"
dpkg -i gitlab-runner_${arch}.deb
```

TODO: Redaction

## Podman à la 

--network = host dans la config

-> Expliquer le problème

Essayer de mettre une IP bidon dans le etc/host et l'utiliser plutôt que 127.0.0.1 qui est casse gueule.

