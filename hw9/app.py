import streamlit as st
from transformers import pipeline
from PIL import Image
import requests
from io import BytesIO

image_url = "https://m.973.com/upload/images/20231016/20231016113714_66230.gif"
response = requests.get(image_url)
image = Image.open(BytesIO(response.content))
st.sidebar.image(image, use_column_width=True)

generator = pipeline('text-generation', model='gpt2')

st.markdown("# My Hugging Face Model Application")
st.markdown("## Please enter your question or prompt below:")

user_input = st.text_input("ciallo (∠· ω ·)⌒★")

if st.button('Generate Text'):
    with st.spinner('Generating text...'):
        outputs = generator(user_input, max_length=100, num_return_sequences=1, truncation=True)
    
    for output in outputs:
        st.markdown("### Generated Text:")
        st.markdown(f"> {output['generated_text']}")