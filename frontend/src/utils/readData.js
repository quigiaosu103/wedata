export const convertJSON = (jsonlines) => {
    let jsonObject = jsonlines.split('\n');

    let data = []
    jsonObject.forEach((line, index) => {
        if(index !== jsonObject.length - 1)
            data.push(JSON.parse(line))
    })

    return data
}