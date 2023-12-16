#!/bin/bash

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
DIR_1="${SCRIPT_DIR}/basic1"
DIR_2="${SCRIPT_DIR}/basic2"

rnd() {
  local path="$1"
  local size="$2"
  if [[ -z "${size}" ]]; then
    size="$(( ( RANDOM % 5000 )  + 100 ))"
  fi
  openssl rand -base64 "${size}" > "${path}"
}

rm -rf "${DIR_1}" "${DIR_2}"

mkdir "${DIR_1}"
pushd "${DIR_1}" > /dev/null
{
  mkdir folder
  rnd folder/asd.id.txt
  rnd folder/uiu.md.txt 2000
  rnd bbb.mv.txt
  rnd ghq.dl.txt
  rnd foo.id.txt
  rnd tst.md.txt 500
  rnd wop.mv.txt
  rnd xyz.id.txt
  rnd za1.dl.txt
}
popd > /dev/null

cp -R "${DIR_1}" "${DIR_2}"
pushd "${DIR_2}" > /dev/null
{
  mv bbb.mv.txt xxx.mv.txt
  mv wop.mv.txt folder/wop.mv.txt
  rm ghq.dl.txt
  rm za1.dl.txt
  rnd tst.md.txt 600
  rnd folder/uiu.md.txt 700
  rnd pqr.ad.txt
  rnd 123.ad.txt
}
popd > /dev/null
