const std = @import("std");
const clap = @import("clap");

const checkCmd = @import("cmd/check.zig").checkCmd;

const main_params = clap.parseParamsComptime(
    \\-h, --help  Display this help and exit.
    \\<command>
    \\
);

const SubCommands = enum {
    help,
    check,
};

const main_parsers = .{
    .command = clap.parsers.enumeration(SubCommands),
};

pub fn main(init: std.process.Init) !void {
    // Init cmd iterator
    var iter = try init.minimal.args.iterateAllocator(init.gpa);
    defer iter.deinit();
    _ = iter.next();

    // Parse
    var diag = clap.Diagnostic{};
    var res = clap.parseEx(clap.Help, &main_params, main_parsers, &iter, .{
        .diagnostic = &diag,
        .allocator = init.gpa,
        .terminating_positional = 0,
    }) catch |err| {
        try diag.reportToFile(init.io, .stderr(), err);
        return err;
    };
    defer res.deinit();

    // Help
    if (res.args.help != 0)
        std.debug.print("--help\n", .{});

    // Subcommands
    const command = res.positionals[0] orelse return error.MissingCommand;
    switch (command) {
        .help => std.debug.print("--help\n", .{}),
        .check => try checkCmd(init.io, init.gpa, &iter),
    }
}
