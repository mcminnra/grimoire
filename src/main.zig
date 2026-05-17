const std = @import("std");
const Io = std.Io;

pub fn main(init: std.process.Init) !void {
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    const arena: std.mem.Allocator = init.arena.allocator();
    defer arena.deinit();

    const args = try init.minimal.args.toSlice(arena);
    for (args) |arg| {
        std.log.info("arg: {s}", .{arg});
    }
}
