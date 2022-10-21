# Prerequisites:
#
# - Log in to the DigitalOcean API, either interactively, by running
#   `doctl auth init`, or by setting the `DIGITALOCEAN_ACCESS_TOKEN`
#   environment variable.

function export_vars_for_environment {
  local environment=$1
  local env_file="crates/collab/k8s/environments/${environment}.sh"
  if [[ ! -f $env_file ]]; then
    echo "Invalid environment name '${environment}'"
    exit 1
  fi
  export $(cat $env_file)
}

function image_id_for_version {
  local version=$1
  if [[ ! ${version} =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Invalid version number '${version}'"
    exit 1
  fi
  TAG_NAMES=$(doctl registry repository list-tags collab --no-header --format Tag)
  if ! $(echo "${TAG_NAMES}" | grep -Fqx v${version}); then
    echo "No such image tag: 'zed/collab:v${version}'"
    echo "Found tags"
    echo "${TAG_NAMES}"
    exit 1
  fi
  
  echo "registry.digitalocean.com/zed/collab:v${version}"
}

function version_for_image_id {
  local image_id=$1
  echo $image_id | cut -d: -f2
}

function target_zed_kube_cluster {
  if [[ $(kubectl config current-context 2> /dev/null) != do-nyc1-zed-1 ]]; then
    doctl kubernetes cluster kubeconfig save zed-1
  fi
}
