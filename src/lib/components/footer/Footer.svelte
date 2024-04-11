<script lang="ts">
    import { Separator } from "$lib/components/ui/separator";

    import * as Avatar from "$lib/components/ui/avatar";
    import * as Dialog from "$lib/components/ui/dialog";

    import { Sparkles, GitBranch, GitFork, BookUser  } from 'lucide-svelte';

    import { onMount } from "svelte";

    let repoData = {
    name: "",
    description: "",
    html_url: "",
    stargazers_count: 0,
  };

  let userData = {
    login: "",
    bio: "",
    avatar_url: "",
    html_url: "",
    public_repos: 0,
    followers: 0
  }


  onMount(async () => {
    const repo_response = await fetch("https://api.github.com/repos/Slackaduts/trebuchet");
    const user_response = await fetch("https://api.github.com/users/Slackaduts")
    repoData = await repo_response.json();
    userData = await user_response.json();
  });
</script>


<Dialog.Root>
    <Dialog.Trigger>
      <h5 class = "font-semibold italic hover:underline p-1">Trebuchet is a free and open-source tool. If you paid for this, you were scammed.</h5>
    </Dialog.Trigger>
    <Dialog.Content class = "max-w-sm">
      <div class = "flex flex-col items-center justify-start">
        <div class = "flex flex-row justify-between space-x-3 w-full p-1 items-center">
          <div class = "flex flex-row space-x-3">
            <GitBranch size = {24}/>
            <h1 class = "font-semibold">{repoData.name}</h1>
          </div>
          <div class = "flex flex-row space-x-1 items-center justify-end">
            <p class = "text-sm font-semibold">{repoData.stargazers_count}</p>
            <Sparkles size = {16}/>
          </div>
        </div>
        
        <span class = "w-full justify-start">
          <p class = "text-sm p-1">{repoData.description}</p>
        </span>
        

        <Separator class = "m-2"/>

        <div class = "flex flex-row justify-between space-x-3 w-full p-1 items-center justify-between">
          <div class = "flex flex-row space-x-3 items-center">
            <Avatar.Root class = "w-12 h-12">
              <Avatar.Image src= {userData.avatar_url} />
              <Avatar.Fallback>SK</Avatar.Fallback>
            </Avatar.Root>
            <h1 class = "font-semibold">{userData.login}</h1>
          </div>

          <div class = "flex flex-col justify-center w-fit items-center">
            <div class = "flex flex-row space-x-1 items-center justify-end p-1">
              <p class = "text-sm font-semibold">{userData.public_repos}</p>
              <GitFork size = {16}/>
            </div>
            <Separator class = ""/>
            <div class = "flex flex-row space-x-1 items-center justify-end p-1">
              <p class = "text-sm font-semibold">{userData.followers}</p>
              <BookUser size = {16}/>
            </div>
          </div>
        </div>

        <span class = "w-full justify-start">
          <p class = "text-sm p-1">{userData.bio}</p>
        </span>
        

      </div>
    </Dialog.Content>
  </Dialog.Root>