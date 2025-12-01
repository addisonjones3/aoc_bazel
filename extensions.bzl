load("//aoc:defs.bzl", "year_repo")

use_year = tag_class(
    attrs = {
        "cookie_file": attr.label(
            default = "//.aoc_cookie",
        ),
        "year": attr.int(),
    },
)

def _aoc_puzzle_input_impl(module_ctx):
    year_repo_names = []
    for mod in module_ctx.modules:
        for year in mod.tags.use_year:
            aoc_cookie = module_ctx.read(year.cookie_file)
            year_repo_name = "aoc{0}".format(year.year)
            year_repo_names.append(year_repo_name)
            year_repo(name = year_repo_name, year = year.year, auth = aoc_cookie)

    return module_ctx.extension_metadata(
        root_module_direct_deps = year_repo_names,
        root_module_direct_dev_deps = [],
        reproducible = True,  # repo state is only a function of the lockfile
    )

aoc_puzzle_input = module_extension(
    implementation = _aoc_puzzle_input_impl,
    tag_classes = {"use_year": use_year},
)
