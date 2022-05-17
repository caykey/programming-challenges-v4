const std = @import("std");
const Vector = @import("vector.zig").Vector;

pub fn main() void {
    const a = Vector(f32, 3).init(.{ 1, 2, 3 });
    const b = Vector(f32, 3).init(.{ 4, 5.5, 6 });

    std.debug.print("Vector a: {} Vector b: {}\n", .{ a, b });
    std.debug.print("Dot product: {} Cross product {}\n", .{ a.dot(b), a.cross(b) });
}
