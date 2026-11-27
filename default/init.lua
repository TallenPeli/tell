-- require("types.lua")
--
-- ============================================
-- YOU ARE USING THE DEFAULT TELL CONFIGURATION
-- ============================================
--
-- Edit this file to get started
--
-- https://github.com/TallenPeli/tell/docs/index.md
--

-- tell.pkgs({ "nvim", "git" })

print("System Arch: ", tell.system.arch)
print("Distro Name: ", tell.system.distro_id)
print("System Package Manager", tell.system.pkg_manager)
