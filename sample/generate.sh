#!/bin/bash

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
DIR_1="${SCRIPT_DIR}/snap1"
DIR_2="${SCRIPT_DIR}/snap2"

rnd() {
  local size="$1"
  if [[ -z "${size}" ]]; then
    size="$(( ( RANDOM % 5000 )  + 100 ))"
  fi
  openssl rand -base64 "${size}"
}

rm -rf "${DIR_1}" "${DIR_2}"

mkdir "${DIR_1}"
pushd "${DIR_1}" > /dev/null
{
  mkdir folder
  echo "$(rnd)" > folder/asd.id.txt
  echo "$(rnd 2000)" > folder/uiu.md.txt
  echo "$(rnd)" > bbb.mv.txt
  echo "$(rnd)" > ghq.dl.txt
  echo "$(rnd)" > foo.id.txt
  echo "$(rnd 500)" > tst.md.txt
  echo "$(rnd)" > wop.mv.txt
  echo "$(rnd)" > xyz.id.txt
  echo "$(rnd)" > za1.dl.txt
}
popd > /dev/null

cp -R "${DIR_1}" "${DIR_2}"
pushd "${DIR_2}" > /dev/null
{
  mv bbb.mv.txt xxx.mv.txt
  mv wop.mv.txt folder/wop.mv.txt
  rm ghq.dl.txt
  rm za1.dl.txt
  echo "$(rnd 600)" > tst.md.txt
  echo "$(rnd 700)" > folder/uiu.md.txt
  echo "$(rnd)" > pqr.ad.txt
  echo "$(rnd)" > 123.ad.txt
}
popd > /dev/null
