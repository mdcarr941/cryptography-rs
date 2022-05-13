# This PyOxidizer configuration file when built will emit files that will
# enable us to link Python into a new binary.

PYTHON_VERSION = VARS.get("PYTHON_VERSION", "3.9")


def make_resources():
    dist = default_python_distribution(python_version=PYTHON_VERSION)

    policy = dist.make_python_packaging_policy()

    policy.extension_module_filter = "all"
    policy.include_distribution_sources = True
    policy.include_distribution_resources = True
    policy.resources_location = "in-memory"

    exe = dist.to_python_executable(
        name="pyoxy",
        packaging_policy=policy,
    )

    return exe.to_embedded_resources()


register_target("resources", make_resources)
resolve_targets()
