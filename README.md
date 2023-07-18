# DevOps Training

My personal training repository for DevOps tools such as GitLab CI/CD, Podman, Kubernetes,
Prometheus and many more.

Currently, my knowledge only covers bits of Software Architecture, Software Development,
Data Engineering, HPC and Artificial Intelligence.

I want to extend it to DevOps tools because these are crucial in an IT organization,
and kinda awesome. And I also want to learn Rust and Go.

Be aware that the tasks listed here may be modified as I advance in the project.

WARNING: This project is currently paused as I work on another private project.
However, during the development of this private project there will be DevOps steps mentioned here. I will then update reports in the current project regarding my DevOps training.

## Reports

Here are the exercises I plan to do in order to train myself.
Each step will be summarized in [reports](reports).

## Tutorials

Once in a while, a sanitized tutorial will be written to summarize a step
in [tutorials](tutorials).

## Steps

### 1. Setting Up a Repository (GitLab)

- [X] Set up a local rootless GitLab repository initialized with this repository's mirror.
- [X] Mirror the GitLab repository to GitHub using SSH.
- [X] Protect the `main` branch to work only using branches.
- [X] Add workflow (TODO/DOING/REVIEW) labels.

### 2. Setting Up DevOps Foundations (CI/CD)

<!-- TODO: Links to the reports everywhere it is possible -->

- [ ] Setup GitLab Runners:
    - [X] On the host (rootless Podman).
    - [ ] Within VMs (VirtualBox).
    - [ ] Within Containers (Podman-in-Podman).
- [ ] Use GitLab CI/CD pipelines to run linting:
    - [X] Markdown
    - [X] Known words
    - [ ] Grammar
    - [X] Find Broken Links in .md files
- [X] Run the linter jobs, if and only, files of their according type have been modified.
- [X] Run the linter jobs only on the files that have been modified.
- [ ] Learn and set up Jenkins

### 3. Building an Application (Rust & Go)

- [ ] Try to import C or C++ libraries in Rust to see how it is done.
- [ ] Create an HTTP server in Go to run a matrix multiplication through HTTP using the
previous library.
- [ ] Use GitLab CI/CD pipelines to run linting:
    - [ ] Rust
    - [ ] Go
- [ ] Add tests coverage
- [X] Add code formatter if possible
- [ ] Make a RESTful API to interact with the application

### 4. Containerize the Application (Podman)

- [X] Build and run this application in a Debian container.

### 5. Gather Metrics

- [ ] Measure MFlops, matrix sizes and steps, HTTP status code and response time and:
    - [ ] Make the HTTP metrics scraped by Prometheus
    - [ ] Push the matrices' metrics to Prometheus gateway
    - [ ] Make the last matrices' metrics available in the response
- [ ] Add logging in the Go server.
    - [ ] Collect the logging using a specialized tool, and also send it to Prometheus.
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
- [ ] Spawn all the above, but all same matrix multiplication should be split over
- [ ] multiple containers, possibly over multiple hosts.
- [ ] Learn Terraform

### 8. Set-up Alerts (Kubernetes & Grafana)

- [ ] Use Grafana to set up alerts on critical errors.
- [ ] Use Kubernetes to set up alerts on critical errors.

### 9. Actually Deploy the Application (AWS & GCP)

- [ ] Learn Terraform
- [ ] Use Terraform to set up an environment from a fresh installation able to run the application

### 10. Actually Deploy the Application (AWS & GCP)

- [ ] Deploy the application in a free Amazon Web Server with restrictions over RAM and
disk usage inside the app'.
    - [ ] Deploy a cluster of the application using Kubernetes.
- [ ] Deploy the application in a free Google Cloud Platform with restrictions over RAM
and disk usage.
- [ ] Run the deployment to both platforms whenever the branch `production` is merged.

### Secure the Application (Kubernetes ?)

- [ ] Add a user/password creation tool and secure the passwords.
- [ ] Add an administrator role that can be attached to users.
- [ ] Use a certificate and HTTPS if possible.
- [ ] Encrypt all the containers, the encryption key must be different for each of them.
- [ ] Encrypt the logging container disk.

## References

### Documentations

- Rust: <https://doc.rust-lang.org/stable/book/>
- Go: <https://go.dev/doc/tutorial/getting-started>
- BLAS: <https://www.openblas.net/>
- LAPACK: <https://github.com/Reference-LAPACK/lapack>
- Podman: <https://docs.podman.io/en/latest/>
- Prometheus: <https://prometheus.io/docs/introduction/overview/>
- Grafana: <https://grafana.com/docs/grafana/latest/getting-started/>
- Kubernetes: <https://kubernetes.io/docs/setup/>

## Useful Links

- CNCF Landscape: <https://landscape.cncf.io/> To know which solutions are most used and hence useful to learn.

### Books

- *Automating DevOps with GitLab CI/CD Pipelines* written by Christopher Cowell, Nicholas Lotz and Chris Timberlake.
