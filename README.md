# DevOps Training

My personal training repository for DevOps tools such as Gitlab CI/CD, Podman, Kubernetes, Prometheus and many more.

Currently, my knowledge only covers bits of Software Architecture, Software Development, Data Engineering, HPC and Artificial Intelligence.

I want to extend it to DevOps tools because these are crucial in an IT organization, and kinda awesome. And I also want to learn Rust and Go.

## Steps

Here are the exercises I plan to do in order to train myself. Each step will be summarized in [reports](reports).

### 1. Setting Up a Repository (GitLab)

- [X] Set up a local rootless GitLab repository initialized with this repository's mirror.
- [X] Mirror the GitLab repository to GitHub using SSH.
- [X] Protect the `main` branch to work only using branches.
- [X] Add workflow (TODO/DOING/REVIEW) labels.

### 2. Setting Up DevOps Foundations (CI/CD)

- [ ] Setup GitLab Runners:
  - [ ] On the host (rootless Podman).
  - [ ] Within VMs (VirtualBox).
  - [ ] Within Containers (Podman-in-Podman).
- [ ] Use GitLab CI/CD pipelines to run linting:
  - [ ] Markdown
- [ ] Run the linter jobs, if and only, files of their according type have been modified.
- [ ] Run the linter jobs only on the files that have been modified.
- [ ] Add code coverage and formatter if possible. Coverage can come later with the application.

### 3. Building an Application (Rust & Go)

- [ ] Create a library of random matrix multiplication in Rust using [BLAS](https://www.openblas.net/) and or [LAPACK](https://github.com/Reference-LAPACK/lapack).
- [ ] Create an HTTP server in Go to run a matrix multiplication through HTTP using the previous library.

### 4. Containerize the Application (Podman)

- [ ] Build and run this application in a Debian container.

### 5. Gather Metrics

- [ ] Measure MFlops, matrix sizes and steps, HTTP status code and response time and:
   - [ ] Make the HTTP metrics scrapable by Prometheus
   - [ ] Push the matrixes metrics to Prometheus gateway
   - [ ] Make the last matrixes metrics available in the response
- [ ] Add logging in the Go server.
  - [ ] Collect the logging using a specialized tools, and also send it to Prometheus.
- [ ] Create Grafana Dashboards and Playlist.

### 6. Deploy the Application and Prometheus (Kubernetes)

- [ ] Use Kubernetes to spawn:
    - The application in a single node
    - A container with Prometheus and Grafana
    - A container for logging management

### 7. Run in Cluster (Kubernetes)

- [ ] Use Kubernetes to spawn:
    - The application in multiple nodes
    - A load-balancer before the application nodes
    - A container with Prometheus and Grafana
    - A container for logging management
- [ ] Spawn all of the above, but all same matrix multiplication should be splitted over multiple containers, possibly over multiple hosts.

### 7. Set-up Alerts (Kubernetes & Grafana)

- [ ] Use Grafana to set-up alerts on critical errors.
- [ ] Use Kubernetes to set-up alerts on critical errors.

### 8. Actually Deploy the Application (AWS & GCP)

- [ ] Deploy the application in a free Amazon Web Server with restrictions over RAM and disk usage inside the app'.
  - [ ] Deploy a cluster of the application using Kubernetes.
- [ ] Deploy the application in a free Google Cloud Platform with restrictions over RAM and disk usage.
- [ ] Run the deployment to both platforms whenever the branch `production` is merged.

### Secure the Application (Kubernetes ?)

- [ ] Add a user/password creation tool and secure the passwords.
- [ ] Add an administrator role that can be attached to users.
- [ ] Use a certificate and HTTPS if possible.
- [ ] Encrypt all of the containers, the encryption key must be different for each of them.
- [ ] Encrypt the logging container disk.

## Documentations

- Rust: <https://doc.rust-lang.org/stable/book/>
- Go: <https://go.dev/doc/tutorial/getting-started>
- BLAS: <https://www.openblas.net/>
- LAPACK: <https://github.com/Reference-LAPACK/lapack>
- Podman: <https://docs.podman.io/en/latest/>
- Prometheus: <https://prometheus.io/docs/introduction/overview/>
- Grafana: <https://grafana.com/docs/grafana/latest/getting-started/>
- Kubernetes: https://kubernetes.io/docs/setup/
