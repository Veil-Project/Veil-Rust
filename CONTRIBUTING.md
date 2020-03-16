<!-- Based on https://raw.githubusercontent.com/rust-lang/rust/master/CONTRIBUTING.md -->
# Contributing to Veil Rust Library
[contributing-to-veil-rust]: #contributing-to-veil-rust

Thank you for your interest in contributing to the Veil Rust Library! There are many ways to 
contribute, and we appreciate all of them. This document is a bit long, so here's 
links to the major sections:

* [Feature Requests](#feature-requests)
* [Bug Reports](#bug-reports)
* [Pull Requests](#pull-requests)
* [Writing Documentation](#writing-documentation)
* [Issue Triage](#issue-triage)

If you have questions, please hop on the [Veil Discord server][veil-discord].

As a reminder, all contributors are expected to follow our [Code of Conduct][coc].

[veil-discord]: https://discord.veil-project.com/
<!-- [coc]: NOTE: need link to COC -->

## Feature Requests
[feature-requests]: #feature-requests

To request a change to the way the Veil Rust Library works, please head over 
to the [Veil Discord server][veil-discord] and find the Veil Rust channel. Please do not post them 
into the issue section of this repository.

## Bug Reports
[bug-reports]: #bug-reports

While bugs are unfortunate, they're a reality in software. We can't fix what we 
don't know about, so please report liberally. If you're not sure if something 
is a bug or not, feel free to file a bug anyway.

**If you believe reporting your bug publicly represents a security risk to users, 
please follow our [instructions for reporting security vulnerabilities](https://github.com/i/need/destination)**.

If you have the chance, before reporting a bug, please [search existing 
issues](https://github.com/i/need/destination), 
as it's possible that someone else has already reported your error. This doesn't 
always work, and sometimes it's hard to know what to search for, so consider this 
extra credit. We won't mind if you accidentally file a duplicate report.

Similarly, to help others who encountered the bug find your issue, consider filing 
an issue with a descriptive title, which contains information that might be unique 
to it. This can be the language or compiler feature used, the conditions that trigger the bug, 
or part of the error message if there is any.

Opening an issue is as easy as following [this link](https://github.com/i/need/destination) 
and filling out the fields.

Here's a template that you can use to file a bug, though it's not necessary to 
use it exactly:

    <short summary of the bug>

    I tried this code:

    <code sample that causes the bug>

    I expected to see this happen: <explanation>

    Instead, this happened: <explanation>

    Software Version:


## Pull Requests
[pull-requests]: #pull-requests

Pull requests are the primary mechanism we use. GitHub itself 
has some [great documentation][about-pull-requests] on using the Pull Request feature. 
We use the "fork and pull" model [described here][development-models], where 
contributors push changes to their personal fork and create pull requests to 
bring those changes into the source repository.

[about-pull-requests]: https://help.github.com/articles/about-pull-requests/
[development-models]: https://help.github.com/articles/about-collaborative-development-models/

Please make pull requests against the `master` branch.

We follow a no merge policy, meaning, when you encounter merge conflicts you are 
expected to always rebase instead of merge.
E.g. always use rebase when bringing the latest changes from the master branch to 
your feature branch. Also, please make sure that fixup commits are squashed into 
other related commits with meaningful commit messages.

Everything must be formatted using the [Rust Format][rustfmt] file.

### About Commits
Commits should be [atomic](https://en.wikipedia.org/wiki/Atomic_commit#Atomic_commit_convention) 
and the diffs should be easy to read. Do not mix any formatting fixes or code 
moves with actual code changes.

Commit messages should be verbose by default consisting of a short subject line 
(50 chars max), a blank line and detailed explanatory text as separate paragraph(s), 
unless the title alone is self-explanatory (like "Corrected typo in init.cpp") in 
which case a single title line is sufficient. Commit messages should be helpful to 
people reading your code in the future, so explain the reasoning for your decisions. 
Further explanation [here](http://chris.beams.io/posts/git-commit/).

If a particular commit references another issue, please add the reference. For 
example: `refs #1234` or `fixes #4321`. Using the `fixes` or `closes` keywords 
will cause the corresponding issue to be closed when the pull request is merged.

#### Squashing commits
If your pull request is accepted for merging, you may be asked by a maintainer 
to squash and or [rebase](https://git-scm.com/docs/git-rebase) your commits 
before it will be merged. The basic squashing workflow is shown below.

    git checkout your_branch_name
    git rebase -i HEAD~n
    # n is normally the number of commits in the pull request.
    # Set commits (except the one in the first line) from 'pick' to 'squash', save and quit.
    # On the next screen, edit/refine commit messages.
    # Save and quit.
    git push -f # (force push to GitHub)

If you have problems with squashing (or other workflows with `git`), you can 
alternatively enable "Allow edits from maintainers" in the right GitHub 
sidebar and ask for help in the pull request.

Please refrain from creating several pull requests for the same change. 
Use the pull request that is already open (or was created earlier) to amend 
changes. This preserves the discussion and review that happened earlier for 
the respective change set.

The length of time required for peer review is unpredictable and will vary from 
pull request to pull request.

[rustfmt]: https://github.com/i/need/destination/rustfmt.toml

## Writing Documentation
[writing-documentation]: #writing-documentation

Documentation improvements are very welcome. The source of `doc.rust-lang.org` 
is located in `src/doc` in the tree, and standard API documentation is generated 
from the source code itself. Documentation pull requests function in the same way 
as other pull requests.

To find documentation-related issues, sort by the [T-doc label][tdoc].

[tdoc]: https://github.com/i/need/place/issues?q=is%3Aopen%20is%3Aissue%20label%3AT-doc

You can find documentation style guidelines in [RFC 1574][rfc1574].

[rfc1574]: https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text

## Issue Triage
[issue-triage]: #issue-triage

Sometimes, an issue will stay open, even though the bug has been fixed. And 
sometimes, the original bug may go stale because something has changed in the 
meantime.

It can be helpful to go through older bug reports and make sure that they are 
still valid. Load up an older issue, double check that it's still true, and 
leave a comment letting us know if it is or is not. The [least recently 
updated sort][lru] is good for finding issues like this.

Contributors with sufficient permissions on the Rust repo can help by adding 
labels to triage issues:

* Yellow, **A**-prefixed labels state which **area** of the project an issue
  relates to.

* Magenta, **B**-prefixed labels identify bugs which are **blockers**.

* Dark blue, **beta-** labels track changes which need to be backported into
  the beta branches.

* Light purple, **C**-prefixed labels represent the **category** of an issue.

* Green, **E**-prefixed labels explain the level of **experience** necessary
  to fix the issue.

* Red, **I**-prefixed labels indicate the **importance** of the issue. The
  [I-nominated][inom] label indicates that an issue has been nominated for
  prioritizing at the next triage meeting.

* The purple **metabug** label marks lists of bugs collected by other
  categories.

* Purple gray, **O**-prefixed labels are the **operating system** or platform
  that this issue is specific to.

* Orange, **P**-prefixed labels indicate a bug's **priority**. These labels
  are only assigned during triage meetings, and replace the [I-nominated][inom]
  label.

* Pink, **regression**-prefixed labels track regressions from stable to the
  release channels.

* The light orange **relnotes** label marks issues that should be documented in
  the release notes of the next release.

* Gray, **S**-prefixed labels are used for tracking the **status** of pull
  requests.

If you're looking for somewhere to start, check out the [E-easy][eeasy] tag.

[inom]: https://github.com/i/need/place/issues?q=is%3Aopen+is%3Aissue+label%3AI-nominated
[eeasy]: https://github.com/i/need/place/issues?q=is%3Aopen+is%3Aissue+label%3AE-easy
[lru]: https://github.com/i/need/place/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc
