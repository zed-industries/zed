#!/bash

# shellcheck disable=2030,2031,2154

set -e -u -o pipefail

# shellcheck disable=2206
declare -a docker_run_options=(${RUN_OR_DOCKER_OPTIONS-})
RUN_OR_DOCKER_PULL=${RUN_OR_DOCKER_PULL:-false}
RUN_OR_DOCKER_PUSH=${RUN_OR_DOCKER_PUSH:-false}

run() (
  verbose=${RUN_OR_DOCKER_VERBOSE:-false}
  bin=$(dirname "${BASH_SOURCE[1]}")
  self=$(basename "${BASH_SOURCE[1]}")
  root=${ROOT:-$PWD}
  prog=$(cd "$bin" && echo "$self".*)
  version=${version:-$(calculate-version)}
  image=yamlio/$self:$version

  if [[ $prog == *'.*' ]]; then
    prog=$self
    lang=bash
  else
    case $prog in
      *.sh) lang=bash ;;
      *.bash) lang=bash ;;
      *.pl) lang=perl ;;
      *.py) lang=python3 ;;
      *.rb) lang=ruby ;;
      *) die "Don't recognize language of '$prog'" ;;
    esac
  fi

  if $RUN_OR_DOCKER_PUSH; then
    build-docker-image
    exit
  fi

  if [[ -e /.dockerenv || ${RUN_OR_DOCKER-} == local ]]; then
    if [[ $self == "$prog" ]]; then
      main "$@"
    else
      run-local "$@"
    fi
    return
  fi

  if [[ ${RUN_OR_DOCKER-} == force* || ${GITHUB_ACTIONS-} ]]; then
    run-docker "$@"
    return
  fi

  out=$(check 2>/tmp/out) && rc=0 || rc=$?

  err=$(< /tmp/out)

  [[ $rc == 0 || $err == FAIL:* ]] || die "Error: $err"

  if [[ $rc -eq 0 ]]; then
    run-local "$@"
  else
    $verbose &&
      echo "Can't run '$self' locally: ${err#FAIL:\ }" >&2
    echo "Running '$self' with docker..." >&2
    run-docker "$@"
  fi
)

run-local() (
  export RUN_OR_DOCKER=local
  "$lang" "$bin/$prog" "$@"
)

run-docker() (
  if [[ ${RUN_OR_DOCKER-} == force-build ]]; then
    build-docker-image
  else
    docker inspect --type=image "$image" &>/dev/null &&
      ok=true || ok=false
    if ! $ok; then
      if $RUN_OR_DOCKER_PULL; then
        (
          $verbose && set -x
          docker pull "$image"
        )
      else
        build-docker-image
      fi
    fi
  fi

  args=()
  docker_run_options+=(
    --env ROOT=/home/host
  )
  for arg; do
    if [[ $arg == "$root"/* ]]; then
      arg=/home/host/${arg#$root/}
    fi
    args+=("$arg")
  done

  flags=()
  [[ -t 0 ]] && flags+=('--tty')

  workdir=/home/host
  [[ ${RUN_OR_DOCKER_WORKDIR-} ]] &&
    workdir=$workdir/$RUN_OR_DOCKER_WORKDIR

  uid=$(id -u)
  gid=$(id -g)

  $verbose && set -x
  docker run "${flags[@]}" --interactive --rm \
    --volume "$root":/home/host \
    --workdir "$workdir" \
    --user "$uid:$gid" \
    --entrypoint '' \
    "${docker_run_options[@]}" \
    "$image" \
    "$self" "${args[@]}"
)

force() {
  fail 'docker is forced here'
}

need() {
  cmd=$1

  [[ $(command -v "$cmd") ]] ||
    fail "requires command '$cmd'"

  [[ ${2-} ]] || return 0

  if [[ ${2-} =~ ^[0-9]+(\.|$) ]]; then
    check=need-version
  else
    check=need-modules
  fi

  "$check" "$@"
}

need-version() {
  cmd=$1 ver=$2

  if [[ $("$cmd" --version) =~ ([0-9]+)\.([0-9]+)(\.[0-9]+)? ]]; then
    set -- "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}" "${BASH_REMATCH[3]#.}"
  else
    die "Could not get version from '$cmd --version'"
  fi

  vers=$ver
  while [[ $vers && $* ]]; do
    v=${vers%%.*}
    if [[ $1 -gt $v ]]; then
      return
    elif [[ $1 -lt $v ]]; then
      fail "requires '$cmd' version '$ver' or higher"
    fi
    if [[ $vers != *.* ]]; then
      return
    fi
    vers=${vers#*.}
    shift
  done
  fail "requires '$cmd' version '$ver' or higher"
}

need-modules() {
  cmd=$1; shift

  for module; do
    case $cmd in
      perl)
        if [[ $module == *=* ]]; then
          want=$module
          version=${module#*=}
          module=${module%=*}
          perl -M"$module"\ "$version" -e1 &>/dev/null ||
            fail "'$cmd' requires Perl module '$want'"
        else
          want=$module
          perl -M"$module" -e1 &>/dev/null ||
            fail "'$cmd' requires Perl module '$module'"
        fi
        ;;
      node)
        node -e "require('$module');" &>/dev/null ||
          fail "'$cmd' requires NodeJS module '$module'"
        ;;
      python)
        python3 -c "import $module" &>/dev/null ||
          fail "'$cmd' requires Python(3) module '$module'"
        ;;
      ruby)
        list=$(gem list)
        if [[ $module == *=* ]]; then
          version=${module#*=}
          module=${module%=*}
        else
          version=0
        fi
        (set -x; ruby -e "gem '$module', '>=$version'") &>/dev/null ||
          fail "'$cmd' requires Ruby module '$module' >= v$version"
        ;;
      *) die "Can't check module '$module' for '$cmd'" ;;
    esac
  done
}

build-docker-image() (
  build=$(mktemp -d)

  build-fail() { fail "docker-build failed: $*"; }

  cmd() (
    _args=${1//\ \+\ /\ &&\ }
    echo "$_args"
    echo
  )

  run() (
    cmd "RUN $*"
  )

  from() {
    _from=$1
    case $_from in
      alpine)
        cmd 'FROM alpine'
        cmd 'WORKDIR /home'
        cmd 'RUN apk update && apk add bash build-base coreutils'
        ;;
      ubuntu)
        cmd 'FROM ubuntu:20.04'
        cmd 'WORKDIR /home'
        cmd 'RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y build-essential'
        ;;
      *) build-fail "from $*"
    esac
  }

  pkg() (
    case $_from in
      alpine)
        cmd "RUN apk add $*"
        ;;
      ubuntu)
        cmd "RUN DEBIAN_FRONTEND=noninteractive apt-get install -y $*"
        ;;
      *) build-fail "pkg $*"
    esac
  )

  cpan() (
    case $_from in
      alpine)
        pkg perl perl-dev perl-app-cpanminus wget
        ;;
      ubuntu)
        pkg cpanminus
        ;;
      *) build-fail "cpan $*"
    esac

    cmd "RUN cpanm -n $*"
  )

  npm() (
    case $_from in
      alpine)
        pkg nodejs npm
        ;;
      *) build-fail "npm $*"
    esac

    cmd "RUN mkdir node_modules && npm install $*"
  )

  gem() (
    case $_from in
      alpine)
        pkg ruby
        ;;
      *) build-fail "npm $*"
    esac

    for module; do
      if [[ $module == *=* ]]; then
        module="${module%=*} -v ${module#*=}"
      fi

      cmd "RUN gem install $module"
    done
  )

  pip() (
    case $_from in
      alpine)
        pkg python3 py3-pip
        ;;
      *) build-fail "npm $*"
    esac

    cmd "RUN pip3 install $*"
  )

  (
    dockerfile
    bin=$(dirname "$0")
    bin=${bin#$root/}
    if [[ $bin == bin ]]; then
      cmd "ENV PATH=/home/host/bin:\$PATH"
    else
      cmd "ENV PATH=/home/host/$bin:/home/host/bin:\$PATH"
    fi
  ) > "$build/Dockerfile"


  (
    set -x
    cd "$build"
    docker build -t "$image" .
  )

  rm -fr "$build"

  if $RUN_OR_DOCKER_PUSH; then
    (
      set -x
      docker push "$image"
    )
  fi
)

calculate-version() (
  [[ ${uses-} ]] ||
    die "'$0' requires either '\$version' variable or '\$uses' array"

  cd "$(dirname "$0")" || exit

  cat "${uses[@]}" |
    md5sum |
    cut -d' ' -f1
)

fail() { echo "FAIL: $*" >&2; exit 1; }
die() { echo "$*" >&2; exit 1; }
