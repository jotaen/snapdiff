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
  openssl rand "${size}" > "${path}"
}

rm -rf "${DIR_1}" "${DIR_2}"

mkdir "${DIR_1}"
pushd "${DIR_1}" > /dev/null
{
  mkdir folder
  rnd identical1.txt 4716
  rnd identical2.txt 8712
  cp identical2.txt identical222.duplicate.txt
  touch identical3.empty.txt
  rnd .identical4.dot 57612
  rnd folder/identical5.txt 911
  touch folder/identical6.empty.txt
  rnd moved1.txt 474
  rnd moved2.txt 60091
  rnd deleted1.txt 38620
  rnd deleted2.txt 1098
  rnd modified1.more.txt 541
  rnd folder/modified2.less.txt 2762
  ln -s identical1.txt link1.txt
  ln -s identical2.txt link2.txt
}
popd > /dev/null

cp -R "${DIR_1}" "${DIR_2}"
pushd "${DIR_2}" > /dev/null
{
  # Move files:
  mv moved1.txt moved111.txt
  mv moved2.txt folder/moved222.mv.txt
  # Delete files:
  rm deleted1.txt
  rm deleted2.txt
  # Modify files:
  rnd modified1.more.txt 90031
  rnd folder/modified2.less.txt 327
  # Add files:
  rnd added1.txt
  rnd added2.txt
  # (Add) Duplicate files:
  cp identical1.txt identical111.duplicate.txt
  # Relink files:
  rm link2.txt && ln -s identical3.empty.txt link2.txt
}
popd > /dev/null
