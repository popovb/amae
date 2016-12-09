# Автомонтирование и запуск (AutoMountAndExec)

## Что это такое и зачем?
Тулза предназначена для автоматического примонтирования файловой
системы подключённого диска и последующего запуска скрипта для манипуляции
с примонтированным разделом (например копирование файлов на съёмный диск).

## Как это работает?
Вся конфигурация лежит в файле правил для udev `85-usb-disk.rules`:
```
ACTION=="add", KERNEL=="sd?1", SUBSYSTEM=="block",
ENV{AMAE_LABEL}="ANC",
ENV{AMAE_DEVICE}="/dev/$name",
ENV{AMAE_MOUNT_DIR}="/mnt/temp",
ENV{AMAE_SCRIPT}="/bin/ls",
ENV{AMAE_UMOUNT}="NO",
RUN{program}="/usr/local/bin/MountAndExec"
```
При появлении в системе блочного устройства (первого раздела), будут выставлены
переменные окружения и будет запущена описываемая программа (`RUN{program}="/usr/local/bin/MountAndExec"`).

### Переменные окружения
* `AMAE_LABEL` - метка раздела будет сравнена со значением этой переменной.
Если значение это входит как подстрока в метку раздела, то будет предпринята
попытка монтирования.
* `AMAE_DEVICE` - путь к устройству раздела.
* `AMAE_MOUNT_DIR` - путь куда монтировать диск.
* `AMAE_SCRIPT` - скрипт/программа, которая будет запущена после упешного монтирования. Первым параметром
будет передан каталог, куда смонтирован раздел.
* `AMAE_UMOUNT` - если в `YES`, то после отработки тулзы, раздел будет отмонтирован. Целесообразно
оставить в `NO`, если `AMAE_SCRIPT` форкается.

### Контроль работы
Лог работы программы можно найти в `syslog`, например:
```
Dec  9 12:30:51 dabbler MountAndExec[7219]: MountAndExec-0.1.0 starting!
Dec  9 12:30:51 dabbler MountAndExec[7219]: env: AMAE_DEVICE -> /dev/sdb1
Dec  9 12:30:51 dabbler MountAndExec[7219]: env: AMAE_MOUNT_DIR -> /mnt/temp
Dec  9 12:30:51 dabbler MountAndExec[7219]: env: AMAE_SCRIPT -> /bin/ls
Dec  9 12:30:51 dabbler MountAndExec[7219]: env: AMAE_LABEL -> ANC
Dec  9 12:30:51 dabbler MountAndExec[7219]: env: AMAE_UMOUNT -> NO
Dec  9 12:30:51 dabbler MountAndExec[7219]: Label of part: labelANClabel
Dec  9 12:30:51 dabbler MountAndExec[7219]: Type of part: ext4
Dec  9 12:30:51 dabbler MountAndExec[7219]: Trying mount part...
Dec  9 12:30:51 dabbler MountAndExec[7219]: ... /dev/sdb1 to /mnt/temp OK!
Dec  9 12:30:51 dabbler MountAndExec[7219]: Trying execute command...
Dec  9 12:30:51 dabbler kernel: [ 9065.597901] EXT4-fs (sdb1): mounted filesystem with ordered data mode. Opts: (null)
Dec  9 12:30:51 dabbler MountAndExec[7219]: ... /bin/ls /mnt/temp OK!
Dec  9 12:30:51 dabbler MountAndExec[7219]: MountAndExec-0.1.0 exiting!
```

## Сборка и установка
1. `sudo aptitude install libblkid-dev`.
2. `cargo build --release`.
3. Положить `MountAndExec` в нужный каталог (например в `/usr/local/bin`).
4. Поправить нужным образом `85-usb-disk.rules` и положить в `/etc/udev/rules.d`.
5. Перечитать правила `sudo udevadm control --reload`.

## Лицензия
MPL 2.0
