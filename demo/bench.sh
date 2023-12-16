#!/bin/bash

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)"
DIR_1="${SCRIPT_DIR}/bench1"
DIR_2="${SCRIPT_DIR}/bench2"

rnd() {
  local file="$1"
  local size="$2"
  dd if=/dev/urandom "of=${file}" "bs=${size}" count=1 2> /dev/null
}

rm -rf "${DIR_1}" "${DIR_2}"

mkdir "${DIR_1}"
pushd "${DIR_1}" > /dev/null
{
  rnd '1.blob' '24m'
  rnd '2.blob' '48m'
  rnd '3.blob' '96m'
  rnd '4.blob' '48m'
  rnd '5.blob' '256m'
  rnd '6.blob' '128m'
  rnd '7.blob' '512m'
  rnd '8.blob' '128m'
  rnd '9.blob' '96m'
}
popd > /dev/null

cp -R "${DIR_1}" "${DIR_2}"
pushd "${DIR_2}" > /dev/null
{
  echo '123' >> '6.blob'
  rm '2.blob'
  mv '8.blob' '888.blob'
  rnd 'new-1.blob' '96m'
  rnd 'new-2.blob' '128m'
}
popd > /dev/null
