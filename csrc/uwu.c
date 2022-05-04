#include <stdio.h>
#include "uwu.h"
#include "lite_xl_plugin_api.h"

static lua_State *L;
static int ready_cb, disconnect_cb, error_cb;
/* UWU usage
	const char* uwu = uwuify("I like to eat cheese becase it is delicious");
*/


static int f_uwuify(lua_State *L) {
	const char *s = luaL_checkstring(L, 1);
	
	const char *uwued = uwuify(s);
	lua_pushstring(L, uwued);
	return 0;
}

static const struct luaL_Reg lib[] = {
	{"uwuify", f_uwuify},
	{NULL, NULL}
};

int luaopen_lite_xl_uwu(lua_State *state, void *XL) {
	lite_xl_plugin_init(XL);
	L = state;
	luaL_newlib(L, lib);
	return 1;
}
