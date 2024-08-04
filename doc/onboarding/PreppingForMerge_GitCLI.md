# Preparing your 'working_branch' for Merge to 'main' Branch

## 0 Scope

This instruction is here to walk you through, step by step, the Git CLI process of preparing your own `working_branch` for merge into the `main` branch. This example uses VSCode for handling rebase merge conflicts that might emerge.

There is a helpful video tutorial [here](https://superluminal.docsend.com/view/chf4zfudcza7ui8v).

You will want to use this instruction _after_ you have:
- Opened a Github PR for your branch
- Received a PR Review and implemented the recommended code changes
- Your deployment checks are all passing (green check marks)
- Your PR checklist is complete
- Your `working_branch` is fully ready for merge into `main`

This instruction set will provide full git commands. ie. Your personal git aliases found in your `.gitconfig` files will not be considered. Translation between these instructional git commands and your own personal aliases will need to be considered separately and individually.

### 0.1 Some notes on Git Aliases
For commonly used GitCLI aliases often utilized by **harlantwood** in pair programming sessions, please check them out [here](https://github.com/harlantwood/dev_env/blob/master/.gitconfig).

If you would like to save any of the commands outlined in this instruction set as an alias of your own, you can either utilize **harlantwood**'s aliases, in the above link, or you can create your own aliases by following [these instructions](https://git-scm.com/book/en/v2/Git-Basics-Git-Aliases). These instructions address how you can add aliases through the command line, but you can also just copy and paste Harlan's (or your own) aliases directly into your `.gitconfig` file in your local directory.


### 0.2 Preparation Process

The preparation process consists of four major steps

1. Finding the *Most Recent Common Ancestor* between your `working_branch` and the `main` branch
2. Squashing all of your `working_branch`'s commits
3. Rebasing your `working_branch` off of the `main` branch
4. Push & Force Release


### 0.3 TL;DR

Just need a brief reminder, not the whole description? Find the short form below, or check out the DocSend video [here](https://superluminal.docsend.com/view/chf4zfudcza7ui8v). Read on to Section 1.0 get the whole step-by-step instruction.

Here's the short-hand:

- [ ] 1. Find Most Recent Common Ancestor SHA
```
git log --oneline --decorate --graph --tags --remotes --branches --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --date=relative
```

- [ ] 2. Squash Commits
```
# Interactive Rebase - Use YOUR common ancestor SHA here
git rebase -i bc157a5
```

- [ ] 3. Rebase off main
```
git rebase origin/main
```

- [ ] 4. Push & Force With Lease
```
git push --tags --set-upstream --force-with-lease origin HEAD
```


## 1. Finding the Most Recent Common Ancestor

The *Most Recent Common Ancestor* is the last Git Commit that your `working_branch` and `main` branch shared. You can think of the *Most Recent Common Ancestor* as the last commit that was made to `main` before you created your `working branch`.

This can be challenging when simply utilizing the basic `git log` command. A more sophisticated, and visually intuitive way to view the log file is by using the following command:

Note: Be sure you have your `working_branch` checked out.
```
git log --oneline --decorate --graph --tags --remotes --branches --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --date=relative
```


### 1.2 Notes on analyzing your log output
In your terminal window:
- Commits along the left-most margin of this log output are on your `working_branch`
- Commits that are indented belong to other branches, one of which should be the `origin/main` branch
- Individual commits are marked with an asteriks -> *


### 1.3 Find the common ancestor SHA
1. Scan the log output for where the ==HEAD== of `origin/main` is located
2. Follow the commit history of `origin/main` (downward) to find where the `origin/main` branch connects back to to your `working_branch`
3. Copy or make note of the commit's SHA (eg.  bc157a5) -> This is the *Most Recent Common Ancestor*


## 2 Squashing all of your `working_branch` commits

In this section we want to condense your `working_branch` commits into a single commit.


### 2.1 Interactive Rebase
To squash all of your `working_branch` commits, the best way to do this is to utilize Git's Interactive Rebase. Here we will walk you through how to do this step by step. We basically want to squash ALL commits on your `working_branch` SINCE the Most Recent Common Ancestor. To do this you will need the SHA you gathered in Step 1.3.

In your terminal, run the following command with <u>your</u> common ancestor SHA:

```
# Interactive Rebase - Use YOUR common ancestor SHA here
git rebase --interactive bc157a5
```


### 2.2 Squashing Commits

Now, inside your terminal, or inside VS Code, a `git-rebase-todo.txt` file automatically pops up. You will see all of the commits on your branch up to, but not including,  the *Most Common Ancestor Commit*. Each commit line will have the word `pick` in front of it.

- Leave the _top commit_ in the text file with the `pick` label. This is the commit we will be squashing all others down <u>into</u>.
- For all other lines below, replace the `pick` label with the word `squash` or the letter `s`
- Save the `git-rebase-todo.txt` file and close it.

Now, a `COMMIT_EDITMSG.txt` file will automatically show up populated with a concatenated list of all the commit messages that were present in the squashed commits. This text file becomes the message detail of your single commit that you squashed all other commits into. You can edit this text file directly, or accept the commit message automatically generated. Save and close the file.


### 2.3 Check your Squash
To check the success of your Squash, re-run the log command we utilized in Step 1.

```
git log --oneline --decorate --graph --tags --remotes --branches --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --date=relative
```

You should now see the singular squashed commit now present on the HEAD of your `working_branch` .


## 3. Rebasing

You have now squashed your commits down into 1, and now want to rebase your `working_branch` off of the `main` branch.


### 3.1 Ensuring 'main' is up-to-date
 Before you rebase, Run:

```
git fetch origin/main
```

This will make sure you are rebasing off of the most up-to-date `main` branch.

Then run the rebase command:

```
git rebase main
```

Helpful hint: You can combine the `fetch` and `rebase` commands above with:

```
git rebase origin/main
```

### 3.2 If Any: Resolve Merge Conflicts
*Skip this Section if you do not have merge-conflicts. Go directly to <u>Section 4: Push & Force With Lease</u> if you do not have merge conflicts.

Using your Diff-editor of choice (OpenDiff, VSCode, etc) resolve any resulting merge conflicts. (Consult with the author(s) of the code changes if you are not certain.)

Once your merge conflicts are resolved, add the changes in your working directory to the Staging area:

```
git add .
```

Now, Continue the Rebase:

```
git rebase --continue
```


### 4 Push & Force With Lease

At this point, before pushing your changes, your local `working_branch` is now out of sync with the remote branch. You can confirm this by checking the Status of your `working _branch`

Run,

```
git status
```

... you should see a message in the terminal stating that your current single-commit local `working_branch` has "diverged" from the previous multi-commit version of your `working_branch` that is on the remote origin. This is not an error. You <u>do not</u> want to do a standard `git push` and `git pull` to resolve the divergence. As in-fact, this divergence was intended, and we want the origin (remote) to now get in alignment with your local branch.

To handle the divergence, You want to instead run:

```
git push --tags --set-upstream --force-with-lease origin HEAD
```


## PREP COMPLETE: Merge into main

The remote `working_branch` is now synced with your local and should only have the one squashed commit. You are now ready to merge your `working_branch` into the `main` branch. You can either request your Git Pull Request Reviewer to merge your branch if you do not have merge privileges, or head to your github Pull Request and kick off the merge yourself.
