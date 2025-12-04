""""""
attrs = {
    "input": attr.label(
        allow_single_file = True,
    ),
    "solver": attr.label(
        cfg = "exec",
        doc = "a Bazel target that takes a single file as input and outputs a single line as output",
        executable = True,
        allow_single_file = True,
    ),
}

def _day_run_macro_impl(name, visibility, solver, input):
    _day_run_rule(name = name, solver = solver, input = input)

day_run = macro(
    implementation = _day_run_macro_impl,
    attrs = attrs,
)

def _day_run_rule_impl(ctx):
    output = ctx.actions.declare_file("{0}.output".format(ctx.attr.name))
    ctx.actions.run(
        executable = ctx.executable.solver,
        outputs = [output],
        inputs = [ctx.file.input],
        arguments = [ctx.file.input.path, output.path],
    )

    return [
        DefaultInfo(files = depset([output])),
    ]

_day_run_rule = rule(
    implementation = _day_run_rule_impl,
    attrs = attrs,
)
