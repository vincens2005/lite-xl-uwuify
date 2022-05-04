-- mod-version:2 lite-xl 2.00
local core = require "core"
local command = require "core.command"
local contextmenu = require "plugins.contextmenu"

local uwuifier = require "plugins.uwuifier.uwu"


command.add("core.docview", {
	["doc:uwuify"] = function()
		local doc = core.active_view.doc
		doc:replace(function(text)
			return uwuifier.uwuify(text), 1
		end)
	end
})

contextmenu:register(
	"core.docview",
	{
		{text = "Uwuify", command = "doc:uwuify"}
	}
)
