import openai

client = openai.OpenAI(
    base_url="http://127.0.0.1:8080/v1",
    api_key="test",
)

completion = client.chat.completions.create(
    messages=[
        {
            "role": "user",
            "content": "This is a test.",
        }
    ],
    model="test",
)

print(completion.choices[0].message.content)