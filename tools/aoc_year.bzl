"""stuff"""
COUNT_DAYS = {
    2025: 12,
}

input_url_base = "https://adventofcode.com/{0}/day/{1}/input"

def _year_repo_impl(ctx):
    ctx.file(
        "BUILD",
        content = """exports_files(glob(["**"]))""",
    )
    count_days = COUNT_DAYS.get(ctx.attr.year)
    if count_days == None:
        count_days = 25
    for day in range(1, count_days + 1):
        result = ctx.execute([
            "curl",
            input_url_base.format(ctx.attr.year, day),
            "-L",  # take redirects
            "--cookie",
            ctx.attr.auth,
        ])

        if result.return_code != 0:
            fail("failed to get input:", result.stderr)

        ctx.file(
            "day{0}.txt".format(day),
            content = result.stdout,
        )

year_repo = repository_rule(
    attrs = {
        "year": attr.int(),
        "auth": attr.string(),
    },
    implementation = _year_repo_impl,
)
