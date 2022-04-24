#!/usr/bin/env bash

if ! clear &> /dev/null
  then
    echo "";
  fi
    echo "";
echo "[1/3] 🔥 Checking Crates.io"
if ! cargo check &> /dev/null
 then
   echo "😖 There was a problem running \"cargo check\""
   exit;
 fi
 echo "👍 Ok [1/3]"

echo "[2/3] ⚡ Building API..."
echo " -  Warning: This API can sometimes take a while to build and of course it depends on the amount of processing."
echo ""

if ! cargo build -q &> /dev/null
 then
   echo "😖 There was a problem running \"cargo run\""
   exit
 fi
  echo "👍 Ok [2/3]"
echo "[3/3] 💻 Preparing command..."
if ! sudo rm -rf /usr/bin/httpinteraction  &> /dev/null
 then
   echo "😖 There was a problem running \"mv\""
   exit
 fi
  echo "."
  echo ".."
if ! sudo mv ./target/debug/httpinteraction /usr/bin  &> /dev/null
 then
   echo "😖 There was a problem running \"mv\""
   exit
 fi
   echo "..."
 echo "👍 Ok [3/3]"
  echo ""

