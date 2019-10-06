package com.sunrisechoir.patchql


import com.apollographql.apollo.api.Operation
import com.apollographql.apollo.api.Response
import com.apollographql.apollo.internal.cache.normalized.ResponseNormalizer
import com.apollographql.apollo.internal.json.InputFieldJsonWriter
import com.apollographql.apollo.internal.json.JsonWriter
import com.apollographql.apollo.response.OperationResponseParser
import com.apollographql.apollo.response.ScalarTypeAdapters
import com.beust.klaxon.JsonArray
import com.beust.klaxon.JsonObject
import com.beust.klaxon.Parser
import com.beust.klaxon.jackson.jackson
import okio.Buffer
import java.math.BigDecimal
import java.util.*

class PatchqlApollo(val params: Params) {

    private val scalarTypeAdapters = ScalarTypeAdapters(Collections.emptyMap())
    private val jsonParser = Parser.jackson()

    @Suppress("UNCHECKED_CAST")
    private val normalizer: ResponseNormalizer<MutableMap<String, Any>>? =
        ResponseNormalizer.NO_OP_NORMALIZER as? ResponseNormalizer<MutableMap<String, Any>>

    fun query(operation: Operation<*, *, *>, cb: (Result<Response<*>>) -> Unit) {

        val queryString = marshalOperation(operation)
        val patchql = PatchqlManager.getInstance(params)
        patchql.query(queryString) { result ->
            cb(result.map { resultString ->
                unMarshalOperation(resultString, operation)
            })
        }
    }

    fun query(operation: Operation<*, *, *>): Response<*> {

        val queryString = marshalOperation(operation)
        val patchql = PatchqlManager.getInstance(params)
        val resultString = patchql.query(queryString)
        return unMarshalOperation(resultString, operation)

    }

    private fun <D1 : Operation.Data, D2, V1 : Operation.Variables> marshalOperation(query: Operation<D1, D2, V1>): String {

        val buffer = Buffer()
        val jsonWriter = JsonWriter.of(buffer)
        jsonWriter.serializeNulls = true
        jsonWriter.beginObject()
        jsonWriter.name("operationName").value(query.name().name())
        jsonWriter.name("variables").beginObject()
        query.variables().marshaller()
            .marshal(InputFieldJsonWriter(jsonWriter, scalarTypeAdapters))
        jsonWriter.endObject()

        jsonWriter.name("query").value(query.queryDocument().replace("\\n", ""))

        jsonWriter.endObject()
        jsonWriter.close()
        return buffer.readByteString().utf8()
    }

    private fun unMarshalOperation(response: String, operation: Operation<*, *, *>): Response<*> {

        val mapper = operation.responseFieldMapper()
        val parser = OperationResponseParser(
            operation, mapper, scalarTypeAdapters,
            normalizer
        )

        val json = jsonParser.parse(response.reader()) as JsonObject
        val mappedJson = mapJsonNumbersToBigDecimal(json)

        return parser.parse(mappedJson)
            .toBuilder()
            .build()
    }

    private fun mapJsonNumbersToBigDecimal(jsn: JsonObject): Map<String, Any?> {
        return jsn.mapValues({ entry ->
            when (val v = entry.value) {
                is Int -> BigDecimal(v)
                is Double -> BigDecimal(v)
                is Long -> BigDecimal(v)
                is JsonArray<*> -> v.map { item -> mapJsonNumbersToBigDecimal(item as JsonObject) }
                is JsonObject -> mapJsonNumbersToBigDecimal(v)
                else -> v
            }
        })
    }

}
