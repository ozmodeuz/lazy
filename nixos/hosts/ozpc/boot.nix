{
  boot = {
    initrd = {
      availableKernelModules = [
        "vmd"
        "xhci_pci"
        "ahci"
        "nvme"
        "usbhid"
        "usb_storage"
        "uas"
        "sd_mod"
      ];
    };
    kernelModules = [
      "kvm-intel"
      "coretemp"
      "nct6775"
    ];
    kernelParams = [
      "quiet"
      "splash"
      "boot.shell_on_fail"
    ];
  };
}
