-- mod-version:2 lite-xl 2.00
local core = require "core"
local command = require "core.command"
local uwuifier = require "plugins.uwuifier.uwu"



command.add(nil, {
	["uwuifier:test"] = function()
		core.log(uwuifier.uwuify("hello world!!!"))
	end
})
