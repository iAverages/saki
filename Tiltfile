# Allowed contexts
if k8s_context() != "kind-saki":
    fail("Tiltfile can only be run in the kind-saki context. Please run `kubectl config use-context kind-saki` and try again.")

load('ext://restart_process', 'docker_build_with_restart')
secret_settings(disable_scrub=True)
update_settings(k8s_upsert_timeout_secs=300)

# Helm
load('ext://helm_resource', 'helm_resource', 'helm_repo')
helm_repo("bitnami", "https://charts.bitnami.com/bitnami", labels=["hide_me"])

# Local utils
load("Tiltfile.util", "unique", "git_checkout", "go_service", "website")

load("Tiltprofile", "app_dir", "profiles", "active_profile")
deps = []
for dep in profiles[active_profile]:
    deps.append(dep)

# Namespaces
load("ext://namespace", "namespace_create")
namespace_create("saki")

# Backend
if "database" in deps:
    svc_deps = ["postgres"]
    deps = unique(deps, svc_deps)
    go_service(
        "backend",
        "cmd/backend-service/*.go",
        app_dir,
        deps=svc_deps,
        sync_paths=["api", "cmd", "internal", "pkg"],
        public_port=8080
    )

