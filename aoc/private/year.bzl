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
            "-fL",  # fail fast, take redirects
            "--cookie",
            ctx.attr.auth,
        ])

        content = ""
        if result.return_code != 0:
            content = result.stderr
        else:
            content = result.stdout

        ctx.file(
            "day{0}.txt".format(day),
            content = content,
        )

year_repo = repository_rule(
    attrs = {
        "auth": attr.string(),
        "year": attr.int(),
    },
    implementation = _year_repo_impl,
)
