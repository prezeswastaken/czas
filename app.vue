<script setup>
const time = ref("coulnd't get time :(");
function updateTime() {
    const now = new Date();
    const hours = String(now.getHours()).padStart(2, "0");
    const minutes = String(now.getMinutes()).padStart(2, "0");
    const timeString = `${hours}:${minutes}`;
    time.value = timeString;
}

onMounted(() => {
    updateTime();
    setInterval(updateTime, 1000);
});
</script>
<template>
    <div class="flex justify-center items-center w-full min-h-screen bg-black">
        <p
            class="font-mono text-white text-[100px] sm:text-[150px] md:text-[200px] lg:text-[250px] xl:text-[350px] 2xl:text-[400px]"
        >
            {{ time }}
        </p>
    </div>
</template>
