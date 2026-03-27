use gh_workflow::{Container, Event, Port, Push, Run, Step, Use, Workflow};
use indoc::{formatdoc, indoc};

use crate::tasks::workflows::runners::{self, Platform};
use crate::tasks::workflows::steps::{
    self, CommonJobConditions, FluentBuilder as _, NamedJob, dependant_job, named,
};
use crate::tasks::workflows::vars;

pub(crate) fn deploy_collab() -> Workflow {
    let style = style();
    let tests = tests(&[&style]);
    let publish = publish(&[&style, &tests]);
    let deploy = deploy(&[&publish]);

    named::workflow()
        .on(Event::default().push(Push::default().add_tag("collab-production")))
        .add_env(("DOCKER_BUILDKIT", "1"))
        .add_job(style.name, style.job)
        .add_job(tests.name, tests.job)
        .add_job(publish.name, publish.job)
        .add_job(deploy.name, deploy.job)
}

fn style() -> NamedJob {
    named::job(
        dependant_job(&[])
            .name("Check formatting and Clippy lints")
            .with_repository_owner_guard()
            .runs_on(runners::LINUX_XL)
            .add_step(steps::checkout_repo().with_full_history())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(steps::cargo_fmt())
            .add_step(steps::clippy(Platform::Linux)),
    )
}

fn tests(deps: &[&NamedJob]) -> NamedJob {
    fn run_collab_tests() -> Step<Run> {
        named::bash("cargo nextest run --package collab --no-fail-fast")
    }

    named::job(
        dependant_job(deps)
            .name("Run tests")
            .runs_on(runners::LINUX_XL)
            .add_service(
                "postgres",
                Container::new("postgres:15")
                    .add_env(("POSTGRES_HOST_AUTH_METHOD", "trust"))
                    .ports(vec![Port::Name("5432:5432".into())])
                    .options(
                        "--health-cmd pg_isready \
                         --health-interval 500ms \
                         --health-timeout 5s \
                         --health-retries 10",
                    ),
            )
            .add_step(steps::checkout_repo().with_full_history())
            .add_step(steps::setup_cargo_config(Platform::Linux))
            .add_step(steps::cache_rust_dependencies_namespace())
            .map(steps::install_linux_dependencies)
            .add_step(steps::cargo_install_nextest())
            .add_step(steps::clear_target_dir_if_large(Platform::Linux))
            .add_step(run_collab_tests()),
    )
}

fn publish(deps: &[&NamedJob]) -> NamedJob {
    fn install_doctl() -> Step<Use> {
        named::uses("digitalocean", "action-doctl", "v2")
            .add_with(("token", vars::DIGITALOCEAN_ACCESS_TOKEN))
    }

    fn sign_into_registry() -> Step<Run> {
        named::bash("doctl registry login")
    }

    fn build_docker_image() -> Step<Run> {
        named::bash(indoc! {r#"
            docker build -f Dockerfile-collab \
              --build-arg "GITHUB_SHA=$GITHUB_SHA" \
              --tag "registry.digitalocean.com/zed/collab:$GITHUB_SHA" \
              .
        "#})
    }

    fn publish_docker_image() -> Step<Run> {
        named::bash(r#"docker push "registry.digitalocean.com/zed/collab:${GITHUB_SHA}""#)
    }

    fn prune_docker_system() -> Step<Run> {
        named::bash("docker system prune --filter 'until=72h' -f")
    }

    named::job(
        dependant_job(deps)
            .name("Publish collab server image")
            .runs_on(runners::LINUX_XL)
            .add_step(install_doctl())
            .add_step(sign_into_registry())
            .add_step(steps::checkout_repo())
            .add_step(build_docker_image())
            .add_step(publish_docker_image())
            .add_step(prune_docker_system()),
    )
}

fn deploy(deps: &[&NamedJob]) -> NamedJob {
    fn install_doctl() -> Step<Use> {
        named::uses("digitalocean", "action-doctl", "v2")
            .add_with(("token", vars::DIGITALOCEAN_ACCESS_TOKEN))
    }

    fn sign_into_kubernetes() -> Step<Run> {
        named::bash(formatdoc! {r#"
            doctl kubernetes cluster kubeconfig save --expiry-seconds 600 {cluster_name}
        "#, cluster_name = vars::CLUSTER_NAME})
    }

    fn start_rollout() -> Step<Run> {
        named::bash(indoc! {r#"
            set -eu
            if [[ $GITHUB_REF_NAME = "collab-production" ]]; then
              export ZED_KUBE_NAMESPACE=production
              export ZED_COLLAB_LOAD_BALANCER_SIZE_UNIT=10
              export ZED_API_LOAD_BALANCER_SIZE_UNIT=2
            elif [[ $GITHUB_REF_NAME = "collab-staging" ]]; then
              export ZED_KUBE_NAMESPACE=staging
              export ZED_COLLAB_LOAD_BALANCER_SIZE_UNIT=1
              export ZED_API_LOAD_BALANCER_SIZE_UNIT=1
            else
              echo "cowardly refusing to deploy from an unknown branch"
              exit 1
            fi

            echo "Deploying collab:$GITHUB_SHA to $ZED_KUBE_NAMESPACE"

            source script/lib/deploy-helpers.sh
            export_vars_for_environment $ZED_KUBE_NAMESPACE

            ZED_DO_CERTIFICATE_ID="$(doctl compute certificate list --format ID --no-header)"
            export ZED_DO_CERTIFICATE_ID
            export ZED_IMAGE_ID="registry.digitalocean.com/zed/collab:${GITHUB_SHA}"

            export ZED_SERVICE_NAME=collab
            export ZED_LOAD_BALANCER_SIZE_UNIT=$ZED_COLLAB_LOAD_BALANCER_SIZE_UNIT
            export DATABASE_MAX_CONNECTIONS=850
            envsubst < crates/collab/k8s/collab.template.yml | kubectl apply -f -
            kubectl -n "$ZED_KUBE_NAMESPACE" rollout status deployment/$ZED_SERVICE_NAME --watch
            echo "deployed ${ZED_SERVICE_NAME} to ${ZED_KUBE_NAMESPACE}"

            export ZED_SERVICE_NAME=api
            export ZED_LOAD_BALANCER_SIZE_UNIT=$ZED_API_LOAD_BALANCER_SIZE_UNIT
            export DATABASE_MAX_CONNECTIONS=60
            envsubst < crates/collab/k8s/collab.template.yml | kubectl apply -f -
            kubectl -n "$ZED_KUBE_NAMESPACE" rollout status deployment/$ZED_SERVICE_NAME --watch
            echo "deployed ${ZED_SERVICE_NAME} to ${ZED_KUBE_NAMESPACE}"
        "#})
    }

    named::job(
        dependant_job(deps)
            .name("Deploy new server image")
            .runs_on(runners::LINUX_XL)
            .add_step(steps::checkout_repo())
            .add_step(install_doctl())
            .add_step(sign_into_kubernetes())
            .add_step(start_rollout()),
    )
}
