function export_vars_for_environment {
  local environment=$1
  local env_file="crates/collab/k8s/environments/${environment}.sh"
  if [[ ! -f $env_file ]]; then
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
  export $(cat $env_file)
}

function image_id_for_version {
  local version=$1

  # Check that version is valid
  if [[ ! ${version} =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Invalid version number '${version}'" >&2
    exit 1
  fi

  # Check that image exists for version
  tag_names=$(doctl registry repository list-tags collab --no-header --format Tag)
  if ! $(echo "${tag_names}" | grep -Fqx v${version}); then
    echo "No docker image tagged for version '${version}'" >&2
    echo "Found images with these tags:" ${tag_names} >&2
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
