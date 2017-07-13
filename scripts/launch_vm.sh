#!/usr/bin/env sh

qemu-system-x86_64 -cdrom archlinux.iso -boot order=d -drive file=disk.raw,format=raw -enable-kvm -m 2G -net nic -net user,smb=/home/florian/src
