# other_dms.md

## lightdm

### General
- Provides DBUS service
- Aquires seats via logind (DBUS) exclusively?!
- 

### Hierarchy
- main.c
    - display\_manager\_new
        - seat_start
            - seat\_real\_start
                - display\_server\_start

## SDDM

### General
- Provides DBUS service