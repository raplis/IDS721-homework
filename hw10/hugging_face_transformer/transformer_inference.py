from transformers import pipeline
import sys
import json

# 指定模型名称和版本
model_name = "distilbert/distilbert-base-uncased-finetuned-sst-2-english"
model_version = "af0f99b"

# 使用 revision 参数指定模型版本
nlp = pipeline("sentiment-analysis", model=model_name, revision=model_version)

# 从命令行参数获取输入文本
input_text = sys.argv[1]

# 进行预测
result = nlp(input_text)

# 打印结果到 stdout
print(json.dumps(result))
