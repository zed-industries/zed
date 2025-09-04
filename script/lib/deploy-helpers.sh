function export_vars_for_environment {
  local environment=$1
  local env_file="crates/collab/k8s/environments/${environment}.sh"
  if [[ ! -f $env_file ]]; then
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
  export $(grep -v '^#' $env_file | grep -v '^[[:space:]]*$')
}

function target_zed_kube_cluster {
  if [[ $(kubectl config current-context 2> /dev/null) != do-nyc1-zed-1 ]]; then
    doctl kubernetes cluster kubeconfig save zed-1
  fi
}

function tag_for_environment {
  if [[ "$1" == "production" ]]; then
    echo "collab-production"
  elif [[ "$1" == "staging" ]]; then
    echo "collab-staging"
  else
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
}

function url_for_environment {
  if [[ "$1" == "production" ]]; then
    echo "https://collab.zed.dev"
  elif [[ "$1" == "staging" ]]; then
    echo "https://collab-staging.zed.dev"
  else
    echo "Invalid environment name '${environment}'" >&2
    exit 1
  fi
}
